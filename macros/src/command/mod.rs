mod argument;
mod info;

use self::argument::{parse_arg_option_type, Argument};
use argument::generate_args;
use info::Info;
use proc_macro2::TokenStream;
use quote::{quote, format_ident};
use syn::{Attribute, Error, ItemFn, Lit, Meta, MetaNameValue, Visibility};

pub fn parse(input: ItemFn) -> syn::Result<TokenStream> {
    let ItemFn {
        attrs,
        mut sig,
        block,
        mut vis
    } = input;

    let mut info = Info::default();
    parse_attributes(&attrs, &mut info)?;

    let description = info.description;
    let ident = sig.ident.clone();
    let name = sig.ident.to_string();


    if sig.asyncness.is_none() {
        return Err(Error::new_spanned(sig, "Function must be asynchronus"));
    }

    vis = Visibility::Inherited;
    sig.ident = format_ident!("invoke");

    let args = argument::parse_args(&sig.inputs)?;
    let fn_closure = generate_closure(&args)?;
    let options = generate_options(&args)?;

    Ok(quote! {
        pub fn #ident<'a>() -> tigra::command::Command<'a> {
            tigra::command::Command::new(#name, #description, #fn_closure)
            #(#options)*
        }

        #vis #sig #block
    })
}

pub fn generate_add_option(name: &str, kind: &TokenStream, required: bool) -> TokenStream {
    // TODO: add description field
    quote!(.add_option(#name, "This is a description", #kind, #required))
}

fn generate_options(args: &[Argument]) -> syn::Result<Vec<TokenStream>> {
    let mut tmp = Vec::new();

    for arg in args {
        let name = arg.ident.to_string();
        let required = arg.option;
        let parse_kind = parse_arg_option_type(arg.ty)?;
        let quote = generate_add_option(&name, &parse_kind, !required);

        tmp.push(quote);
    }

    Ok(tmp)
}

fn generate_closure(
    args: &[argument::Argument],
) -> syn::Result<TokenStream> {
    let (idents, tokens) = generate_args(args)?;

    Ok(quote! {
        |ctx, interaction| {
            Box::pin(async move {
                #(#tokens)*
                invoke(ctx, &interaction, #( #idents, )*).await;
            })
        }
    })
}

fn parse_attributes(attrs: &[Attribute], command_info: &mut Info) -> syn::Result<()> {
    for attr in attrs {
        let ident = attr.path.get_ident().unwrap();
        let name = ident.to_string();

        if name.as_str() == "description" {
            let meta = attr.parse_meta()?;
            command_info.description = parse_description(&meta)?;
        } else {
            return Err(Error::new_spanned(attr, "Unknown attribute"));
        }
    }

    assert!(
        !command_info.description.is_empty(),
        "Missing command description"
    );

    Ok(())
}

fn parse_description(meta: &syn::Meta) -> syn::Result<String> {
    if let Meta::NameValue(MetaNameValue {
        lit: Lit::Str(lit_str),
        ..
    }) = meta
    {
        let val = lit_str.value();
        let val_len = val.trim().len();

        if val_len < 1 {
            return Err(Error::new_spanned(
                &meta,
                "Description must be atleast a character long",
            ));
        } else if val_len > 100 {
            return Err(Error::new_spanned(
                &meta,
                "Description must be less than 100 characters",
            ));
        }

        return Ok(val);
    }

    Err(Error::new_spanned(&meta, "Unable to parse description"))
}
