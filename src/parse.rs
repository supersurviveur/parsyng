use core::iter::Peekable;
use std::collections::VecDeque;

use parsyng_quote::ToTokens;

use crate::{
    error::Result,
    proc_macro::{Span, TokenStream, TokenTree, token_stream::IntoIter},
};

#[derive(Clone)]
pub struct ParseBuffer {
    inner: Peekable<IntoIter>,
    depth: u16,
    forks: Vec<VecDeque<TokenTree>>,
}

impl ParseBuffer {
    #[must_use]
    pub fn new(inner: crate::proc_macro::TokenStream) -> Self {
        Self {
            inner: inner.into_iter().peekable(),
            depth: 0,
            forks: vec![VecDeque::new(); 2],
        }
    }

    pub fn span(&mut self) -> Span {
        self.peek().map_or(Span::call_site(), |tt| tt.span())
    }

    pub fn is_empty(&mut self) -> bool {
        self.peek().is_none()
    }

    pub fn peek(&mut self) -> Option<&TokenTree> {
        self.forks[self.depth as usize + 1]
            .front()
            .or_else(|| self.inner.peek())
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

    pub fn try_advance<T: Parse, F: FnOnce(&mut Self) -> Result<T>>(&mut self, f: F) -> Option<T> {
        self.depth += 1;
        self.forks.push(VecDeque::new());
        match f(self) {
            Ok(ok) => {
                self.depth -= 1;
                // Drop the current saved tokens
                // If we are not on the main stream, put these tokens in the parent fork
                let last = self.forks.pop().unwrap();
                if self.depth != 0 {
                    let currents = self.forks.pop().unwrap();
                    self.forks.push(VecDeque::new());
                    self.forks[self.depth as usize - 1].extend(currents);
                    self.forks[self.depth as usize - 1].extend(last);
                }
                Some(ok)
            }
            Err(_) => {
                self.depth -= 1;
                // Drop the current saved tokens
                // If we are not on the main stream, put these tokens in the parent fork
                let last = self.forks.pop().unwrap();
                if self.depth != 0 {
                    self.forks.pop().unwrap();
                    self.forks.push(VecDeque::new());
                    self.forks[self.depth as usize - 1].extend(last);
                }
                None
            }
        }
    }

    pub fn try_parse<T: Parse>(&mut self) -> Option<T> {
        self.try_advance(T::parse)
    }

    pub fn parse<T: Parse>(&mut self) -> Result<T> {
        dbg!(&self.forks);
        dbg!(&self.depth);
        dbg!(&self.inner.peek());
        T::parse(self)
    }

    /// Same as [Self::parse], but guaranteed that if the parsing fails, the stream didn't advanced.
    pub fn peek_token<T: Peek>(&mut self) -> Result<T> {
        T::parse(self)
    }
}

impl Iterator for ParseBuffer {
    type Item = TokenTree;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.forks[self.depth as usize + 1]
            .pop_front()
            .or_else(|| self.inner.next());

        // If we are not on the main stream, push the token in the parent fork.
        if self.depth != 0
            && let Some(ref tt) = result
        {
            self.forks[self.depth as usize].push_back(tt.clone());
        }

        result
    }
}

pub trait Parse: Sized {
    fn parse(input: &mut ParseBuffer) -> Result<Self>;
}

pub trait Peek: Parse {}

#[derive(Clone, Default, Debug)]
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
