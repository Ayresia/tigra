use proc_macro2::TokenStream;
use quote::quote;
use syn::{Attribute, Block, Error, ItemFn, Lit, Meta, MetaNameValue};

#[derive(Default, Debug)]
pub struct CommandInfo {
    pub description: String,
}

pub fn parse_command(input: ItemFn) -> syn::Result<TokenStream> {
    let ItemFn {
        attrs, sig, block, ..
    } = input;

    let mut command_info = CommandInfo::default();

    parse_attributes(&attrs, &mut command_info)?;

    let description = command_info.description;
    let ident = sig.ident.clone();
    let name = sig.ident.to_string();

    if sig.asyncness.is_none() {
        let error = Error::new_spanned(sig, "Function must be asynchronus");
        return Err(error);
    }

    let fn_closure = generate_closure(&block);

    Ok(quote! {
        pub fn #ident<'a>() -> tigra::command::Command<'a> {
            tigra::command::Command::new(#name, #description, #fn_closure)
        }
    })
}

fn generate_closure(block: &Block) -> TokenStream {
    quote! {
        |ctx, interaction| {
            Box::pin(async move #block)
        }
    }
}

fn parse_attributes(attrs: &[Attribute], command_info: &mut CommandInfo) -> syn::Result<()> {
    for attr in attrs {
        let ident = attr.path.get_ident().unwrap();
        let name = ident.to_string();

        match name.as_str() {
            "description" => {
                let meta = attr.parse_meta()?;
                command_info.description = parse_description(meta)?;
            }
            _ => {
                let error = Error::new_spanned(attr, "Unknown attribute");
                return Err(error);
            }
        };
    }

    if command_info.description.is_empty() {
        panic!("Missing command description");
    }

    Ok(())
}

fn parse_description(meta: syn::Meta) -> syn::Result<String> {
    if let Meta::NameValue(MetaNameValue {
        lit: Lit::Str(lit_str),
        ..
    }) = meta
    {
        return Ok(lit_str.value());
    }

    let error = Error::new_spanned(meta, "Unable to parse description");
    Err(error)
}
