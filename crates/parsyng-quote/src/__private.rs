pub use super::*;

pub fn push_lit(stream: proc_macro::TokenStream, tokens: &mut proc_macro::TokenStream) {
    tokens.extend(core::iter::once(proc_macro::Group::new(
        proc_macro::Delimiter::None,
        stream,
    )));
}
pub fn push_group(
    delimiter: proc_macro::Delimiter,
    stream: proc_macro::TokenStream,
    tokens: &mut proc_macro::TokenStream,
) {
    tokens.extend(core::iter::once(proc_macro::Group::new(delimiter, stream)));
}
pub fn push_ident(ident: &str, tokens: &mut proc_macro::TokenStream) {
    tokens.extend(core::iter::once(proc_macro::Ident::new(
        ident,
        proc_macro::Span::call_site(),
    )));
}
pub fn push_ident_raw(ident: &str, tokens: &mut proc_macro::TokenStream) {
    tokens.extend(core::iter::once(proc_macro::Ident::new_raw(
        ident,
        proc_macro::Span::call_site(),
    )));
}
pub fn push_punct_alone(punct: char, tokens: &mut proc_macro::TokenStream) {
    tokens.extend(core::iter::once(proc_macro::Punct::new(
        punct,
        proc_macro::Spacing::Alone,
    )));
}
pub fn push_punct_joint(punct: char, tokens: &mut proc_macro::TokenStream) {
    tokens.extend(core::iter::once(proc_macro::Punct::new(
        punct,
        proc_macro::Spacing::Joint,
    )));
}
