use core::iter::Peekable;
use std::collections::VecDeque;

use parsyng_quote::ToTokens;

use crate::{
    error::Result,
    proc_macro::{Span, TokenStream, TokenTree},
};

#[derive(Debug, Clone, Copy)]
pub enum ParseBufferState {
    Saving,
    Advancing,
    Restoring,
}

#[derive(Clone)]
pub struct ParseBuffer {
    inner: Peekable<crate::proc_macro::token_stream::IntoIter>,
    saved: VecDeque<TokenTree>,
    state: ParseBufferState,
}

impl ParseBuffer {
    #[must_use]
    pub fn new(inner: crate::proc_macro::TokenStream) -> Self {
        Self {
            inner: inner.into_iter().peekable(),
            saved: VecDeque::new(),
            state: ParseBufferState::Advancing,
        }
    }

    pub fn span(&mut self) -> Span {
        self.peek().map_or(Span::call_site(), |tt| tt.span())
    }

    pub fn is_empty(&mut self) -> bool {
        self.peek().is_none()
    }

    pub fn peek(&mut self) -> Option<&TokenTree> {
        match self.state {
            ParseBufferState::Saving => self.inner.peek(),
            ParseBufferState::Restoring => {
                if let Some(tt) = self.saved.front() {
                    Some(tt)
                } else {
                    // self.state = ParseBufferState::Advancing;
                    self.inner.peek()
                }
            }
            ParseBufferState::Advancing => self.inner.peek(),
        }
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
        match self.peek_group() {
            Some(_) => match self.next().unwrap() {
                TokenTree::Group(group) => Some(group),
                _ => None,
            },
            None => None,
        }
    }
    pub fn ident(&mut self) -> Option<crate::proc_macro::Ident> {
        match self.peek_ident() {
            Some(_) => match self.next().unwrap() {
                TokenTree::Ident(ident) => Some(ident),
                _ => None,
            },
            None => None,
        }
    }
    pub fn literal(&mut self) -> Option<crate::proc_macro::Literal> {
        match self.peek_literal() {
            Some(_) => match self.next().unwrap() {
                TokenTree::Literal(literal) => Some(literal),
                _ => None,
            },
            None => None,
        }
    }
    pub fn punct(&mut self) -> Option<crate::proc_macro::Punct> {
        match self.peek_punct() {
            Some(_) => match self.next().unwrap() {
                TokenTree::Punct(punct) => Some(punct),
                _ => None,
            },
            None => None,
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

pub struct Nothing;

impl Parse for Nothing {
    #[inline]
    fn parse(_input: &mut ParseBuffer) -> Result<Self> {
        Ok(Self)
    }
}

impl ToTokens for Nothing {
    #[inline]
    fn to_tokens(&self, _tokens: &mut TokenStream) {}
}
