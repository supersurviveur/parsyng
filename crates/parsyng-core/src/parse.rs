use core::iter;

use crate::ToTokens;

use crate::error::Diagnostics;
use crate::{
    error::Result,
    proc_macro::{Group, Ident, Punct, Span, TokenStream, TokenTree, token_stream::IntoIter},
};

#[derive(Clone)]
pub struct ParseBuffer {
    last_span: Span,
    inner: iter::Peekable<IntoIter>,
}

impl ParseBuffer {
    /// Create a new buffer from a token stream.
    #[must_use]
    pub fn new(inner: crate::proc_macro::TokenStream) -> Self {
        let mut inner = inner.into_iter().peekable();
        Self {
            last_span: inner.peek().map_or_else(Span::call_site, TokenTree::span),
            inner,
        }
    }

    /// Span of the next token, or the last consumed token if the stream is empty.
    pub fn span(&mut self) -> Span {
        let last_span = self.last_span;
        self.peek().map_or(last_span, TokenTree::span)
    }

    /// Return `true` when no tokens remain.
    pub fn is_empty(&mut self) -> bool {
        self.peek().is_none()
    }

    /// Inspect the next token without consuming it.
    pub fn peek(&mut self) -> Option<&TokenTree> {
        self.inner.peek()
    }

    /// Inspect the next token as a group without consuming it.
    pub fn peek_group(&mut self) -> Option<&Group> {
        self.peek().and_then(|token| match token {
            TokenTree::Group(group) => Some(group),
            _ => None,
        })
    }
    /// Inspect the next token as an identifier without consuming it.
    pub fn peek_ident(&mut self) -> Option<&crate::proc_macro::Ident> {
        self.peek().and_then(|token| match token {
            TokenTree::Ident(ident) => Some(ident),
            _ => None,
        })
    }
    /// Inspect the next token as punctuation without consuming it.
    pub fn peek_punct(&mut self) -> Option<&crate::proc_macro::Punct> {
        self.peek().and_then(|token| match token {
            TokenTree::Punct(punct) => Some(punct),
            _ => None,
        })
    }
    /// Inspect the next token as a literal without consuming it.
    pub fn peek_literal(&mut self) -> Option<&crate::proc_macro::Literal> {
        self.peek().and_then(|token| match token {
            TokenTree::Literal(literal) => Some(literal),
            _ => None,
        })
    }
    /// Consume and return the next group token.
    pub fn group(&mut self) -> Option<Group> {
        match self.peek_group() {
            Some(_) => match unsafe { self.next().unwrap_unchecked() } {
                TokenTree::Group(group) => Some(group),
                _ => None,
            },
            None => None,
        }
    }
    /// Consume and return the next identifier token.
    pub fn ident(&mut self) -> Option<crate::proc_macro::Ident> {
        match self.peek_ident() {
            Some(_) => match unsafe { self.next().unwrap_unchecked() } {
                TokenTree::Ident(ident) => Some(ident),
                _ => None,
            },
            None => None,
        }
    }
    /// Consume and return the next identifier when it matches a predicate.
    pub fn ident_and<F: FnOnce(&Ident) -> bool>(
        &mut self,
        f: F,
    ) -> Option<crate::proc_macro::Ident> {
        match self.peek_ident() {
            Some(ident) if f(ident) => match unsafe { self.next().unwrap_unchecked() } {
                TokenTree::Ident(ident) => Some(ident),
                _ => None,
            },
            _ => None,
        }
    }
    /// Consume and return the next literal token.
    pub fn literal(&mut self) -> Option<crate::proc_macro::Literal> {
        match self.peek_literal() {
            Some(_) => match unsafe { self.next().unwrap_unchecked() } {
                TokenTree::Literal(literal) => Some(literal),
                _ => None,
            },
            None => None,
        }
    }
    /// Consume and return the next punctuation token.
    pub fn punct(&mut self) -> Option<crate::proc_macro::Punct> {
        match self.peek_punct() {
            Some(_) => match unsafe { self.next().unwrap_unchecked() } {
                TokenTree::Punct(punct) => Some(punct),
                _ => None,
            },
            None => None,
        }
    }
    /// Consume and return the next punctuation token when it matches a predicate.
    pub fn punct_and<F: FnOnce(&Punct) -> bool>(
        &mut self,
        f: F,
    ) -> Option<crate::proc_macro::Punct> {
        match self.peek_punct() {
            Some(punct) if f(punct) => match unsafe { self.next().unwrap_unchecked() } {
                TokenTree::Punct(punct) => Some(punct),
                _ => None,
            },
            _ => None,
        }
    }

    /// Try a parse on a cloned cursor and commit the result only on success.
    ///
    /// # Errors
    /// If the argument function `f` returns an error, this error is returned.
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

    /// Try to parse a value without consuming input on failure.
    ///
    /// # Errors
    /// Return an error if parsing fails.
    pub fn try_parse<T: Parse>(&mut self) -> Result<T> {
        self.try_advance(T::parse)
    }

    /// Parse a value from the current cursor.
    ///
    /// # Errors
    /// Return an error if parsing fails.
    pub fn parse<T: Parse>(&mut self) -> Result<T> {
        T::parse(self)
    }

    /// Parse a value without advancing the input on failure.
    ///
    /// # Errors
    /// Return an error if parsing fails.
    pub fn peek_parse<T: Peek>(&mut self) -> Result<T> {
        T::parse(self)
    }
}

impl Iterator for ParseBuffer {
    type Item = TokenTree;

    fn next(&mut self) -> Option<Self::Item> {
        match self.inner.next() {
            Some(tt) => {
                self.last_span = tt.span();
                Some(tt)
            }
            None => None,
        }
    }
}

impl ToTokens for ParseBuffer {
    fn to_tokens(&self, tokens: &mut crate::proc_macro::TokenStream) {
        tokens.extend(self.clone());
    }
}

pub trait Parse {
    /// # Errors
    /// Return an error if parsing fails.
    fn parse(input: &mut ParseBuffer) -> Result<Self>
    where
        Self: Sized;
}

/// Marker trait for type that can be parsed without advancing the cursor in case of failure.
pub trait Peek: Parse {}

/// Wrapper to implement [Peek] any type wich implements [Parse] by cloning the [`ParseBuffer`] when parsing.
pub struct Peekable<T> {
    inner: T,
}

impl<T> Peekable<T> {
    /// Consume the wrapper and return the parsed value.
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

/// A placeholder type that parses and prints nothing.
pub type Nothing = ();

impl Parse for Nothing {
    #[inline]
    fn parse(_input: &mut ParseBuffer) -> Result<Self> {
        Ok(())
    }
}

impl ToTokens for Nothing {
    #[inline]
    fn to_tokens(&self, _tokens: &mut TokenStream) {}
}

impl<T: Parse> Parse for Box<T> {
    fn parse(input: &mut ParseBuffer) -> Result<Self> {
        Ok(Self::new(input.parse()?))
    }
}

/// A sentinel type that always fails to parse.
#[derive(Clone, Default, Debug)]
pub struct Invalid;

impl Parse for Invalid {
    #[inline]
    fn parse(input: &mut ParseBuffer) -> Result<Self> {
        Err(Diagnostics::new_error_spanned(
            "Invalid cannot be parsed",
            input.span(),
        ))
    }
}

impl ToTokens for Invalid {
    #[inline]
    fn to_tokens(&self, _tokens: &mut TokenStream) {
        unimplemented!("`Invalid` can not be converted to tokens")
    }
}
