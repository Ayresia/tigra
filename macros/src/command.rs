use std::collections::HashMap;

use proc_macro2::{TokenStream, Ident};
use quote::quote;
use syn::{Attribute, Block, Error, FnArg, ItemFn, Lit, Meta, MetaNameValue, Type};

#[derive(Default, Debug)]
pub struct Info {
    pub description: String,
}

pub fn parse(input: ItemFn) -> syn::Result<TokenStream> {
    let ItemFn {
        attrs, sig, block, ..
    } = input;

    let mut info = Info::default();

    parse_attributes(&attrs, &mut info)?;

    let description = info.description;
    let ident = sig.ident.clone();
    let name = sig.ident.to_string();

    if sig.asyncness.is_none() {
        let error = Error::new_spanned(sig, "Function must be asynchronus");
        return Err(error);
    }

    let args = parse_args(&sig.inputs);
    let fn_closure = generate_closure(&block, &args);

    Ok(quote! {
        pub fn #ident<'a>() -> tigra::command::Command<'a> {
            tigra::command::Command::new(#name, #description, #fn_closure)
        }
    })
}

fn parse_args(
    fn_args: &syn::punctuated::Punctuated<FnArg, syn::token::Comma>,
) -> HashMap<&Ident, String> {
    let mut tmp = HashMap::new();

    for arg in fn_args {
        if let FnArg::Typed(pat_type) = arg {
            let arg_name = if let syn::Pat::Ident(pat_ident) = &*pat_type.pat {
                &pat_ident.ident
            } else {
                unimplemented!()
            };

            let pat_type = &*pat_type.ty;

            let arg_type = match pat_type {
                Type::Path(_) => path_to_string(pat_type),
                Type::Reference(ty_ref) => path_to_string(&ty_ref.elem),
                _ => unimplemented!(),
            };

            tmp.insert(arg_name, arg_type);
        }
    }

    tmp
}

fn path_to_string(pat_type: &Type) -> String {
    if let Type::Path(type_path) = pat_type {
        let path = &type_path.path;
        return path.get_ident().unwrap().to_string();
    }

    unimplemented!()
}

fn generate_closure(block: &Block, args: &HashMap<&Ident, String>) -> TokenStream {
    let args: Vec<&Ident> = args.iter().map(|(k, _)| *k).collect();

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
            let error = Error::new_spanned(attr, "Unknown attribute");
            return Err(error);
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
            let error = Error::new_spanned(&meta, "Description must be atleast a character long");

            return Err(error);
        } else if val_len > 100 {
            let error = Error::new_spanned(&meta, "Description must be less than 100 characters");

            return Err(error);
        }

        return Ok(val);
    }

    let error = Error::new_spanned(&meta, "Unable to parse description");
    Err(error)
}
