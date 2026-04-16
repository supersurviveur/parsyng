use crate::error::Diagnostics;
use crate::parse::Parse;
use crate::proc_macro::{Group, Ident, Literal, Punct, TokenStream, TokenTree};

impl Parse for TokenStream {
    fn parse(input: &mut crate::parse::ParseBuffer) -> crate::error::Result<Self> {
        Ok(input.collect())
    }
}

impl Parse for TokenTree {
    fn parse(input: &mut crate::parse::ParseBuffer) -> crate::error::Result<Self> {
        input.next().ok_or(Diagnostics::new_error(""))
    }
}
impl Parse for Group {
    fn parse(input: &mut crate::parse::ParseBuffer) -> crate::error::Result<Self> {
        input.group().ok_or(Diagnostics::new_error(""))
    }
}
impl Parse for Ident {
    fn parse(input: &mut crate::parse::ParseBuffer) -> crate::error::Result<Self> {
        input.ident().ok_or(Diagnostics::new_error(""))
    }
}
impl Parse for Literal {
    fn parse(input: &mut crate::parse::ParseBuffer) -> crate::error::Result<Self> {
        input.literal().ok_or(Diagnostics::new_error(""))
    }
}
impl Parse for Punct {
    fn parse(input: &mut crate::parse::ParseBuffer) -> crate::error::Result<Self> {
        input.punct().ok_or(Diagnostics::new_error(""))
    }
}
