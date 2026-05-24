use crate::error::{Diagnostics, Result};
use crate::parse::{Parse, ParseBuffer};
use crate::proc_macro::{Group, Ident, Literal, Punct, TokenStream, TokenTree};

impl Parse for TokenStream {
    fn parse(input: &mut ParseBuffer) -> Result<Self> {
        Ok(input.collect())
    }
}

impl Parse for TokenTree {
    fn parse(input: &mut ParseBuffer) -> Result<Self> {
        input.next().ok_or(Diagnostics::new_error_spanned(
            "Expected TokenTree",
            input.span(),
        ))
    }
}
impl Parse for Group {
    fn parse(input: &mut ParseBuffer) -> Result<Self> {
        input.group().ok_or(Diagnostics::new_error_spanned(
            "Expected Group",
            input.span(),
        ))
    }
}
impl Parse for Ident {
    fn parse(input: &mut ParseBuffer) -> Result<Self> {
        input.ident().ok_or(Diagnostics::new_error_spanned(
            "Expected identifier",
            input.span(),
        ))
    }
}
impl Parse for Literal {
    fn parse(input: &mut ParseBuffer) -> Result<Self> {
        input.literal().ok_or(Diagnostics::new_error_spanned(
            "Expected literal",
            input.span(),
        ))
    }
}
impl Parse for Punct {
    fn parse(input: &mut ParseBuffer) -> Result<Self> {
        input.punct().ok_or(Diagnostics::new_error_spanned(
            "Expected punctuation",
            input.span(),
        ))
    }
}
