use proc_macro2::Ident;
use syn::{punctuated::Punctuated, token::Comma, FnArg, Type};

pub struct Argument<'a> {
    pub ident: &'a Ident,
    pub ty: &'a Type,
    pub is_ref: bool,
}

pub fn parse_args(fn_args: &Punctuated<FnArg, Comma>) -> syn::Result<Vec<Argument>> {
    let mut tmp = Vec::new();

    for arg in fn_args {
        if let FnArg::Typed(pat_type) = arg {
            let mut is_ref = false;

            let ident = if let syn::Pat::Ident(pat_ident) = &*pat_type.pat {
                &pat_ident.ident
            } else {
                return Err(syn::Error::new_spanned(pat_type, "Expecting a ident"));
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
