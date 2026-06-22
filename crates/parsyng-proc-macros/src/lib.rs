use parsyng_core as parsyng;

use parsyng_core::quote;
use proc_macro::TokenStream;

mod derive_parse;
mod derive_to_tokens;
mod proc_macro_helper;

pub(crate) fn dbg_macros() -> TokenStream {
    quote! {
        parsyng::debug_stream(&output);
    }
}

#[proc_macro_attribute]
pub fn proc_macro(_args: TokenStream, input: TokenStream) -> TokenStream {
    match proc_macro_helper::proc_macro(_args, input) {
        Ok(ok) => ok,
        Err(err) => {
            let mut tokens = TokenStream::new();
            parsyng_core::ToTokens::to_tokens(&err, &mut tokens);
            tokens
        }
    }
}

#[proc_macro_derive(Parse)]
pub fn derive_parse(input: TokenStream) -> TokenStream {
    match derive_parse::derive_parse(input) {
        Ok(ok) => ok,
        Err(err) => {
            let mut tokens = TokenStream::new();
            parsyng_core::ToTokens::to_tokens(&err, &mut tokens);
            tokens
        }
    }
}

#[proc_macro_derive(ToTokens)]
pub fn derive_to_tokens(input: TokenStream) -> TokenStream {
    match derive_to_tokens::derive_to_tokens(input) {
        Ok(ok) => ok,
        Err(err) => {
            let mut tokens = TokenStream::new();
            parsyng_core::ToTokens::to_tokens(&err, &mut tokens);
            tokens
        }
    }
}
