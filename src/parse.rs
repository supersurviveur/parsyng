use core::iter;

use parsyng_quote::ToTokens;

use crate::{
    error::Result,
    proc_macro::{Group, Ident, Punct, Span, TokenStream, TokenTree, token_stream::IntoIter},
};

#[derive(Clone)]
pub struct ParseBuffer {
    inner: iter::Peekable<IntoIter>,
}

impl ParseBuffer {
    #[must_use]
    pub fn new(inner: crate::proc_macro::TokenStream) -> Self {
        Self {
            inner: inner.into_iter().peekable(),
        }
    }

    pub fn span(&mut self) -> Span {
        self.peek().map_or(Span::call_site(), |tt| tt.span())
    }

    pub fn is_empty(&mut self) -> bool {
        self.peek().is_none()
    }

    pub fn peek(&mut self) -> Option<&TokenTree> {
        self.inner.peek()
    }

    pub fn peek_group(&mut self) -> Option<&Group> {
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
    pub fn group(&mut self) -> Option<Group> {
        match self.peek_group() {
            Some(_) => match self.next().unwrap() {
                TokenTree::Group(group) => Some(group),
                _ => None,
            },
            None => None,
        }
    }
    // pub fn group_and_then<T, F: FnOnce(&Group) -> Result<Option<T>>>(
    //     &mut self,
    //     f: F,
    // ) -> Option<(Group, T)> {
    //     match self.peek_group() {
    //         Some(group) => match f(group) {
    //             Ok(value) => Some((self.group().unwrap(), value)),
    //             Err(e) => e,
    //         },
    //         _ => None,
    //     }
    // }
    pub fn ident(&mut self) -> Option<crate::proc_macro::Ident> {
        match self.peek_ident() {
            Some(_) => match self.next().unwrap() {
                TokenTree::Ident(ident) => Some(ident),
                _ => None,
            },
            None => None,
        }
    }
    pub fn ident_and<F: FnOnce(&Ident) -> bool>(
        &mut self,
        f: F,
    ) -> Option<crate::proc_macro::Ident> {
        match self.peek_ident() {
            Some(ident) if f(ident) => match self.next().unwrap() {
                TokenTree::Ident(ident) => Some(ident),
                _ => None,
            },
            _ => None,
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
    pub fn punct_and<F: FnOnce(&Punct) -> bool>(
        &mut self,
        f: F,
    ) -> Option<crate::proc_macro::Punct> {
        match self.peek_punct() {
            Some(punct) if f(punct) => match self.next().unwrap() {
                TokenTree::Punct(punct) => Some(punct),
                _ => None,
            },
            _ => None,
        }
    }

    pub fn try_advance<T: Parse, F: FnOnce(&mut Self) -> Result<T>>(&mut self, f: F) -> Result<T> {
        let mut fork = self.clone();
        match f(&mut fork) {
            Ok(ok) => {
                *self = fork;
                Ok(ok)
            }
            Err(e) => Err(e),
        }
    }

    pub fn try_parse<T: Parse>(&mut self) -> Result<T> {
        self.try_advance(T::parse)
    }

    pub fn parse<T: Parse>(&mut self) -> Result<T> {
        T::parse(self)
    }

    /// Same as [Self::parse], but guaranteed that if the parsing fails, the stream didn't advanced.
    pub fn peek_parse<T: Peek>(&mut self) -> Result<T> {
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

pub trait Peek: Parse {}

pub struct Peekable<T> {
    inner: T,
}

impl<T> Peekable<T> {
    pub fn inner(self) -> T {
        self.inner
    }
}

impl<T: Parse> Parse for Peekable<T> {
    fn parse(input: &mut ParseBuffer) -> Result<Self> {
        Ok(Self {
            inner: input.try_parse()?,
        })
    }
}

impl<T: Parse> Peek for Peekable<T> {}

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

impl<T: Parse> Parse for Box<T> {
    fn parse(input: &mut ParseBuffer) -> Result<Self> {
        Ok(Box::new(input.parse()?))
    }
}
