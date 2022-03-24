mod argument;
mod info;

use info::Info;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Attribute, Block, Error, ItemFn, Lit, Meta, MetaNameValue};

pub fn parse(input: ItemFn) -> syn::Result<TokenStream> {
    let ItemFn {
        attrs, sig, block, ..
    } = input;

    let mut info = info::Info::default();

    parse_attributes(&attrs, &mut info)?;

    let description = info.description;
    let ident = sig.ident.clone();
    let name = sig.ident.to_string();

    if sig.asyncness.is_none() {
        return Err(Error::new_spanned(sig, "Function must be asynchronus"));
    }

    let args = argument::parse_args(&sig.inputs)?;
    let fn_closure = generate_closure(&block, &args);

    Ok(quote! {
        pub fn #ident<'a>() -> tigra::command::Command<'a> {
            tigra::command::Command::new(#name, #description, #fn_closure)
        }
    })
}

fn generate_closure(block: &Block, args: &[argument::Argument]) -> TokenStream {
    let args: Vec<TokenStream> = args
        .iter()
        .map(|arg| {
            let ident = &arg.ident;
            let ty = arg.ty;

            if arg.is_ref {
                return quote!(#ident: &#ty);
            }

            quote!(#ident: #ty)
        })
        .collect();

    quote! {
        |#(#args,)*| {
            Box::pin(async move #block)
        }
    }
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
