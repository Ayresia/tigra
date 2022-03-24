use proc_macro2::Ident;
use syn::{punctuated::Punctuated, token::Comma, FnArg, Type, TypePath};

pub struct Argument<'a> {
    pub ident: &'a Ident,
    pub ty: &'a Type,
    pub reference: bool,
    pub option: bool,
}

pub fn parse_args(fn_args: &Punctuated<FnArg, Comma>) -> syn::Result<Vec<Argument>> {
    let mut tmp = Vec::new();

    for arg in fn_args {
        if let FnArg::Typed(pat_type) = arg {
            let mut is_ref = false;
            let mut option = false;

            let ident = if let syn::Pat::Ident(pat_ident) = &*pat_type.pat {
                &pat_ident.ident
            } else {
                return Err(syn::Error::new_spanned(pat_type, "Expecting a ident"));
            };

            let pat_type = &*pat_type.ty;
            let ty = match pat_type {
                Type::Path(ty_path) => {
                    option = check_path_option(ty_path);
                    pat_type
                }
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

            tmp.push(Argument {
                ident,
                ty,
                reference: is_ref,
                option,
            });
        }
    }

    Ok(tmp)
}

fn check_path_option(ty_path: &TypePath) -> bool {
    let path = &ty_path.path;
    let ident = &path.segments[0].ident;

    ident.to_string().starts_with("Option")
}
