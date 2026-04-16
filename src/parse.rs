use std::hint::unreachable_unchecked;

use crate::{
    error::{Diagnostic, Diagnostics, Result},
    proc_macro::TokenTree,
};
use itertools::{PeekNth, peek_nth};

pub struct ParseBuffer {
    inner: PeekNth<crate::proc_macro::token_stream::IntoIter>,
}

impl ParseBuffer {
    #[must_use]
    pub fn new(inner: crate::proc_macro::TokenStream) -> Self {
        Self {
            inner: peek_nth(inner),
        }
    }

    pub fn peek(&mut self) -> Option<&TokenTree> {
        self.inner.peek()
    }

    pub fn peek_group(&mut self) -> Option<&crate::proc_macro::Group> {
        self.peek().and_then(|token| match token {
            TokenTree::Group(group) => Some(group),
            _ => None,
        })
    }
    pub fn peek_ident(&mut self) -> Option<&crate::proc_macro::Ident> {
        self.peek().and_then(|token| match token {
            TokenTree::Ident(ident) => Some(ident),
            _ => None,
        })
    }
    pub fn peek_punct(&mut self) -> Option<&crate::proc_macro::Punct> {
        self.peek().and_then(|token| match token {
            TokenTree::Punct(punct) => Some(punct),
            _ => None,
        })
    }
    pub fn peek_literal(&mut self) -> Option<&crate::proc_macro::Literal> {
        self.peek().and_then(|token| match token {
            TokenTree::Literal(literal) => Some(literal),
            _ => None,
        })
    }
    pub fn group(&mut self) -> Option<crate::proc_macro::Group> {
        if self.peek_group().is_some() {
            unsafe {
                match self.next().unwrap_unchecked() {
                    TokenTree::Group(group) => Some(group),
                    _ => unreachable_unchecked(),
                }
            }
        } else {
            None
        }
    }
    pub fn ident(&mut self) -> Option<crate::proc_macro::Ident> {
        if self.peek_ident().is_some() {
            unsafe {
                match self.next().unwrap_unchecked() {
                    TokenTree::Ident(ident) => Some(ident),
                    _ => unreachable_unchecked(),
                }
            }
        } else {
            None
        }
    }

    pub fn parse<T: Parse>(&mut self) -> Result<T> {
        T::parse(self)
    }
}

impl Iterator for ParseBuffer {
    type Item = TokenTree;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

pub trait Parse: Sized {
    fn parse(input: &mut ParseBuffer) -> Result<Self>;
}

impl Parse for u8 {
    fn parse(input: &mut ParseBuffer) -> Result<Self> {
        if let Some(next) = input.next()
            && let TokenTree::Literal(literal) = next
            && let Ok(parsed) = literal.to_string().parse()
        {
            return Ok(parsed);
        }
        Err(Diagnostics::new(Diagnostic::Error))
    }
}
