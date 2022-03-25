use crate::util::{self, option_to_type};
use proc_macro2::{Ident, TokenStream};
use quote::quote;
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

                    if option {
                        option_to_type(pat_type)
                    } else {
                        pat_type
                    }
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

macro_rules! generate_parse_arg {
    ($name: ident, $enum: ident, $ty: ident) => {
        // TODO: add user
        match $name.as_str() {
            "String" => Ok(quote!(
                serenity::model::interactions::application_command::$enum::String
            )),
            "i64" => Ok(quote!(
                serenity::model::interactions::application_command::$enum::Integer
            )),
            "bool" => Ok(quote!(
                serenity::model::interactions::application_command::$enum::Boolean
            )),
            "PartialChannel" => Ok(quote!(
                serenity::model::interactions::application_command::$enum::Channel
            )),
            "f64" => Ok(quote!(
                serenity::model::interactions::application_command::$enum::Number
            )),
            _ => Err(syn::Error::new_spanned($ty, "Unknown type")),
        }
    };
}

pub fn parse_arg_option_value(ty: &Type) -> syn::Result<TokenStream> {
    let name = util::type_to_string(ty);
    generate_parse_arg!(name, ApplicationCommandInteractionDataOptionValue, ty)
}

pub fn parse_arg_option_type(ty: &Type) -> syn::Result<TokenStream> {
    let name = util::type_to_string(ty);
    generate_parse_arg!(name, ApplicationCommandOptionType, ty)
}

pub fn generate_args(args: &[Argument]) -> syn::Result<Vec<TokenStream>> {
    let mut test_vec = vec![];

    if args.is_empty() {
        return Ok(test_vec);
    }

    for (idx, arg) in args.iter().enumerate() {
        let arg_name = arg.ident;
        let arg_type = arg.ty;
        let option_value = parse_arg_option_value(arg_type)?;

        let quote_required = quote! {
            let #arg_name = if let #option_value(val) = interaction
                .data
                .options
                .get(#idx)
                .unwrap()
                .resolved
                .as_ref()
                .unwrap() {
                val
            } else {
                unimplemented!();
            };
        };

        let quote_optional = quote! {
            let #arg_name = if let Some(#arg_name) = interaction.data.options.get(#idx) {
                if let Some(option) = &#arg_name.resolved {
                    if let #option_value(val) = option {
                        (val)
                    } else {
                        None
                    }
                } else {
                    None
                }
            } else {
                None
            };
        };

        if arg.option {
            test_vec.push(quote_optional);
            continue;
        }

        test_vec.push(quote_required);
    }

    Ok(test_vec)
}

fn check_path_option(ty_path: &TypePath) -> bool {
    let path = &ty_path.path;
    let ident = &path.segments[0].ident;

    ident.to_string().starts_with("Option")
}
