use proc_macro2::TokenStream;
use syn::{Attribute, Error, ItemFn, Lit, Meta, MetaNameValue};

#[derive(Default, Debug)]
pub struct CommandInfo {
    pub description: String,
}

pub fn parse_command(input: ItemFn) -> syn::Result<TokenStream> {
    let ItemFn { attrs, .. } = input;
    let mut command_info = CommandInfo::default();

    parse_attributes(&attrs, &mut command_info)?;
    unimplemented!();
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
