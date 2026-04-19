use std::borrow::Cow;

use parsyng_quote::proc_macro::TokenStream;
use proc_macro::Span;

pub fn derive_parse(_input: TokenStream) -> Result<TokenStream, (Cow<'static, str>, Span)> {
    todo!()
}
