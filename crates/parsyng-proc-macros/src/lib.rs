use parsyng_quote::parsyng_spanned;
use proc_macro::TokenStream;

// To parse things more easily, a part of the top-level `parsyng` crate is used as bootstrap.
// Files are behind symlinks.
// It require some hacky `use` and `mod` to avoid any errors.
mod proc_macro {
    pub(crate) use ::proc_macro::*;
}
mod bootstrap;
use bootstrap::*;

#[macro_use]
mod parsing_helpers;
mod derive_parse;
mod proc_macro_helper;

#[proc_macro_attribute]
pub fn proc_macro(_args: TokenStream, input: TokenStream) -> TokenStream {
    match proc_macro_helper::proc_macro(_args, input) {
        Ok(ok) => ok,
        Err((err, span)) => parsyng_spanned! { span =>
            compile_error! { #err }
        },
    }
}

#[proc_macro_derive(Parse)]
pub fn derive_parse(input: TokenStream) -> TokenStream {
    match derive_parse::derive_parse(input) {
        Ok(ok) => ok,
        Err((err, span)) => parsyng_spanned! { span =>
            compile_error! { #err }
        },
    }
}
