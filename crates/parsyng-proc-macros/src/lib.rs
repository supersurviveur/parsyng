#![deny(
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo,
    // rustdoc::all,
    rustdoc::redundant_explicit_links,
    invalid_doc_attributes,
    unused_doc_comments,
    // missing_docs
)]
// We need to add `.into()` due to the `proc-macro2` feature.
#![allow(clippy::useless_conversion)]

use parsyng_core as parsyng;

use parsyng_core::quote;
use proc_macro::{Span, TokenStream};

mod derive_parse;
mod derive_to_tokens;
mod proc_macro_attribute_helper;
mod proc_macro_derive_helper;
mod proc_macro_helper;

pub(crate) fn dbg_macros(
    macro_name: &parsyng_core::proc_macro::Ident,
) -> parsyng_core::proc_macro::TokenStream {
    let location = &format!(
        "{}:{}:{}",
        Span::call_site().file(),
        Span::call_site().line(),
        Span::call_site().column()
    );
    quote! {
        parsyng::debug_stream(#{ macro_name.to_string() }, #location, &output);
    }
}

#[proc_macro_attribute]
pub fn proc_macro(args: TokenStream, input: TokenStream) -> TokenStream {
    match proc_macro_helper::proc_macro(args.into(), input.into()) {
        Ok(ok) => ok,
        Err(err) => parsyng_core::ToTokens::to_token_stream(&err),
    }
    .into()
}

// Export with an underscore, since it will conflicts with the `proc_macro_attribute` builtin.
#[proc_macro_attribute]
pub fn proc_macro_attribute_(args: TokenStream, input: TokenStream) -> TokenStream {
    match proc_macro_attribute_helper::proc_macro_attribute(args.into(), input.into()) {
        Ok(ok) => ok,
        Err(err) => parsyng_core::ToTokens::to_token_stream(&err),
    }
    .into()
}

// Export with an underscore, since it will conflicts with the `proc_macro_derive` builtin.
#[proc_macro_attribute]
pub fn proc_macro_derive_(args: TokenStream, input: TokenStream) -> TokenStream {
    match proc_macro_derive_helper::proc_macro_derive(args.into(), input.into()) {
        Ok(ok) => ok,
        Err(err) => parsyng_core::ToTokens::to_token_stream(&err),
    }
    .into()
}

#[proc_macro_derive(Parse)]
pub fn derive_parse(input: TokenStream) -> TokenStream {
    match derive_parse::derive_parse(input.into()) {
        Ok(ok) => ok,
        Err(err) => parsyng_core::ToTokens::to_token_stream(&err),
    }
    .into()
}

#[proc_macro_derive(ToTokens)]
pub fn derive_to_tokens(input: TokenStream) -> TokenStream {
    match derive_to_tokens::derive_to_tokens(input.into()) {
        Ok(ok) => ok,
        Err(err) => parsyng_core::ToTokens::to_token_stream(&err),
    }
    .into()
}
