extern crate proc_macro;
mod command;

use command::parse_command;
use proc_macro::TokenStream;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn command(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);

    parse_command(input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}
