use parsyng_core as parsyng;

use parsyng_core::quote;
use proc_macro::{Ident, TokenStream};

mod derive_parse;
mod derive_to_tokens;
mod proc_macro_attribute_helper;
mod proc_macro_derive_helper;
mod proc_macro_helper;

pub(crate) fn dbg_macros(macro_name: &Ident, location: String) -> TokenStream {
    quote! {
        parsyng::debug_stream(#{ macro_name.to_string() }, #location, &output);
    }
}

#[proc_macro_attribute]
pub fn proc_macro(args: TokenStream, input: TokenStream) -> TokenStream {
    match proc_macro_helper::proc_macro(args, input) {
        Ok(ok) => ok,
        Err(err) => parsyng_core::ToTokens::to_token_stream(&err),
    }
}

// Export with an underscore, since it will conflicts with the `proc_macro_attribute` builtin.
#[proc_macro_attribute]
pub fn proc_macro_attribute_(args: TokenStream, input: TokenStream) -> TokenStream {
    match proc_macro_attribute_helper::proc_macro_attribute(args, input) {
        Ok(ok) => ok,
        Err(err) => parsyng_core::ToTokens::to_token_stream(&err),
    }
}

// Export with an underscore, since it will conflicts with the `proc_macro_derive` builtin.
#[proc_macro_attribute]
pub fn proc_macro_derive_(args: TokenStream, input: TokenStream) -> TokenStream {
    match proc_macro_derive_helper::proc_macro_derive(args, input) {
        Ok(ok) => ok,
        Err(err) => parsyng_core::ToTokens::to_token_stream(&err),
    }
}

#[proc_macro_derive(Parse)]
pub fn derive_parse(input: TokenStream) -> TokenStream {
    match derive_parse::derive_parse(input) {
        Ok(ok) => ok,
        Err(err) => parsyng_core::ToTokens::to_token_stream(&err),
    }
}

#[proc_macro_derive(ToTokens)]
pub fn derive_to_tokens(input: TokenStream) -> TokenStream {
    match derive_to_tokens::derive_to_tokens(input) {
        Ok(ok) => ok,
        Err(err) => parsyng_core::ToTokens::to_token_stream(&err),
    }
}
