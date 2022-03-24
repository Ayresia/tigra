use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{Attribute, Block, Error, FnArg, ItemFn, Lit, Meta, MetaNameValue, Type};

#[derive(Default, Debug)]
pub struct Info {
    pub description: String,
}

pub struct Argument<'a> {
    pub ident: &'a Ident,
    pub ty: &'a Type,
    pub is_ref: bool,
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

    let args = parse_args(&sig.inputs)?;
    let fn_closure = generate_closure(&block, &args);

    Ok(quote! {
        pub fn #ident<'a>() -> tigra::command::Command<'a> {
            tigra::command::Command::new(#name, #description, #fn_closure)
        }
    })
}

fn parse_args(
    fn_args: &syn::punctuated::Punctuated<FnArg, syn::token::Comma>,
) -> syn::Result<Vec<Argument>> {
    let mut tmp = Vec::new();

    for arg in fn_args {
        if let FnArg::Typed(pat_type) = arg {
            let mut is_ref = false;

            let ident = if let syn::Pat::Ident(pat_ident) = &*pat_type.pat {
                &pat_ident.ident
            } else {
                return Err(syn::Error::new_spanned(
                    pat_type,
                    "Expecting a ident",
                ))
            };

            let pat_type = &*pat_type.ty;
            let ty = match pat_type {
                Type::Path(_) => pat_type,
                Type::Reference(ty_ref) => {
                    is_ref = true;
                    &*ty_ref.elem
                }
                _ => {
                    return Err(syn::Error::new_spanned(
                        pat_type,
                        "Expecting a path or a reference",
                    ))
                }
            };

            tmp.push(Argument { ident, ty, is_ref });
        }
    }

    Ok(tmp)
}

fn generate_closure(block: &Block, args: &[Argument]) -> TokenStream {
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
