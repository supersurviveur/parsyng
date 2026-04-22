use proc_macro::TokenStream;

// To parse things more easily, a part of the top-level `parsyng` crate is used as bootstrap.
// Files are behind symlinks.
// It require some hacky `use` and `mod` to avoid any errors.
mod proc_macro {
    pub(crate) use ::proc_macro::*;
}
#[macro_use]
mod bootstrap;
use bootstrap::*;

mod derive_parse;
mod derive_to_tokens;
mod proc_macro_helper;

pub(crate) fn dbg_macros() -> TokenStream {
    use parsyng_quote::parsyng;

    parsyng! {
        parsyng::debug_stream(&output);
    }
}

#[proc_macro_attribute]
pub fn proc_macro(_args: TokenStream, input: TokenStream) -> TokenStream {
    match proc_macro_helper::proc_macro(_args, input) {
        Ok(ok) => ok,
        Err(err) => {
            let mut tokens = TokenStream::new();
            parsyng_quote::ToTokens::to_tokens(&err, &mut tokens);
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
            parsyng_quote::ToTokens::to_tokens(&err, &mut tokens);
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
            parsyng_quote::ToTokens::to_tokens(&err, &mut tokens);
            tokens
        }
    }
}
