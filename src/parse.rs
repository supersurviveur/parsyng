use core::iter::Peekable;

use crate::{error::Result, proc_macro::TokenTree};

pub struct ParseBuffer {
    inner: Peekable<crate::proc_macro::token_stream::IntoIter>,
}

impl ParseBuffer {
    #[must_use]
    pub fn new(inner: crate::proc_macro::TokenStream) -> Self {
        Self {
            inner: inner.into_iter().peekable(),
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
        self.next().and_then(|token| match token {
            TokenTree::Group(group) => Some(group),
            _ => None,
        })
    }
    pub fn ident(&mut self) -> Option<crate::proc_macro::Ident> {
        self.next().and_then(|token| match token {
            TokenTree::Ident(ident) => Some(ident),
            _ => None,
        })
    }
    pub fn literal(&mut self) -> Option<crate::proc_macro::Literal> {
        self.next().and_then(|token| match token {
            TokenTree::Literal(literal) => Some(literal),
            _ => None,
        })
    }
    pub fn punct(&mut self) -> Option<crate::proc_macro::Punct> {
        self.next().and_then(|token| match token {
            TokenTree::Punct(punct) => Some(punct),
            _ => None,
        })
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
