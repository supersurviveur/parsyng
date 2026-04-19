use crate::proc_macro::{Span, TokenStream};
use parsyng_quote::{ToTokens, parsyng_spanned};

#[derive(Debug, Clone)]
pub enum Diagnostic {
    Error(String, Span),
}

#[derive(Debug, Clone)]
pub struct Diagnostics(Vec<Diagnostic>);

impl Diagnostics {
    #[must_use]
    pub fn empty() -> Self {
        Self(vec![])
    }
    #[must_use]
    pub fn new(diagnostic: Diagnostic) -> Self {
        Self(vec![diagnostic])
    }
    #[must_use]
    pub fn new_error<T: Into<String>>(error: T) -> Self {
        Self::new(Diagnostic::Error(error.into(), Span::call_site()))
    }
    #[must_use]
    pub fn new_error_spanned<T: Into<String>>(error: T, span: Span) -> Self {
        Self::new(Diagnostic::Error(error.into(), span))
    }
}

pub type Result<T> = core::result::Result<T, Diagnostics>;

impl ToTokens for Diagnostic {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(match self {
            Diagnostic::Error(error, span) => parsyng_spanned! { span =>
                compile_error!{ #{ error } }
            },
        });
    }
}
impl ToTokens for Diagnostics {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.0.iter().for_each(|diagnostic| {
            diagnostic.to_tokens(tokens);
        });
    }
}
