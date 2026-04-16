use crate::proc_macro::TokenStream;
use parsyng_quote::{ToTokens, parsyng};

#[derive(Debug, Clone)]
pub enum Diagnostic {
    Error(String),
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
        Self::new(Diagnostic::Error(error.into()))
    }
}

pub type Result<T> = core::result::Result<T, Diagnostics>;

impl ToTokens for Diagnostic {
    fn to_tokens(&self, _tokens: &mut TokenStream) {
        match self {
            Diagnostic::Error(error) => parsyng! {
                compile_error!(#{ error })
            },
        };
    }
}
impl ToTokens for Diagnostics {
    fn to_tokens(&self, _tokens: &mut TokenStream) {
        self.0
            .iter()
            .fold(crate::proc_macro::TokenStream::new(), |acc, _diagnostic| {
                acc
            });
    }
}
