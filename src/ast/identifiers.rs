pub use crate::proc_macro::Ident;

/// https://doc.rust-lang.org/reference/identifiers.html#railroad-IDENTIFIER_OR_KEYWORD
pub(crate) fn is_identifier_or_keyword(ident: &str) -> bool {
    let mut chars = ident.chars();
    chars
        .next()
        .is_some_and(|c| c == '_' || unicode_ident::is_xid_start(c))
        && chars.all(unicode_ident::is_xid_continue)
}
