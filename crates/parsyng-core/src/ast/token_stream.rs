use crate::ToTokens;

use crate::error::{Diagnostics, Result};
use crate::parse::{Parse, ParseBuffer};
use crate::proc_macro::{Group, Ident, Literal, Punct, TokenStream, TokenTree};

impl Parse for TokenStream {
    fn parse(input: &mut ParseBuffer) -> Result<Self> {
        Ok(input.collect())
    }
}

impl Parse for TokenTree {
    fn parse(input: &mut ParseBuffer) -> Result<Self> {
        input.next().ok_or(Diagnostics::new_error_spanned(
            "Expected TokenTree",
            input.span(),
        ))
    }
}
impl Parse for Group {
    fn parse(input: &mut ParseBuffer) -> Result<Self> {
        input.group().ok_or(Diagnostics::new_error_spanned(
            "Expected Group",
            input.span(),
        ))
    }
}
impl Parse for Ident {
    fn parse(input: &mut ParseBuffer) -> Result<Self> {
        input.ident().ok_or(Diagnostics::new_error_spanned(
            "Expected identifier",
            input.span(),
        ))
    }
}
impl Parse for Literal {
    fn parse(input: &mut ParseBuffer) -> Result<Self> {
        input.literal().ok_or(Diagnostics::new_error_spanned(
            "Expected literal",
            input.span(),
        ))
    }
}
impl Parse for Punct {
    fn parse(input: &mut ParseBuffer) -> Result<Self> {
        input.punct().ok_or(Diagnostics::new_error_spanned(
            "Expected punctuation",
            input.span(),
        ))
    }
}

#[derive(Clone, Debug)]
pub struct TokenStreamUntilSemicolon {
    tokens: TokenStream,
}

impl TokenStreamUntilSemicolon {
    pub fn tokens(&self) -> &TokenStream {
        &self.tokens
    }
}

impl Parse for TokenStreamUntilSemicolon {
    fn parse(input: &mut ParseBuffer) -> Result<Self> {
        let mut tokens = TokenStream::new();
        while let Some(token) = input.peek() {
            if matches!(token, TokenTree::Punct(punct) if punct.as_char() == ';') {
                break;
            }
            tokens.extend(Some(input.next().expect("peeked token must exist")));
        }
        Ok(Self { tokens })
    }
}

impl ToTokens for TokenStreamUntilSemicolon {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.tokens.to_tokens(tokens);
    }
}

#[derive(Clone, Debug)]
pub struct TokenStreamUntilComma {
    tokens: TokenStream,
}

impl TokenStreamUntilComma {
    pub fn tokens(&self) -> &TokenStream {
        &self.tokens
    }
}

impl Parse for TokenStreamUntilComma {
    fn parse(input: &mut ParseBuffer) -> Result<Self> {
        let mut tokens = TokenStream::new();
        while let Some(token) = input.peek() {
            if matches!(token, TokenTree::Punct(punct) if punct.as_char() == ',') {
                break;
            }
            tokens.extend(Some(input.next().expect("peeked token must exist")));
        }
        Ok(Self { tokens })
    }
}

impl ToTokens for TokenStreamUntilComma {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.tokens.to_tokens(tokens);
    }
}

#[derive(Clone, Debug)]
pub struct TokenStreamUntilCommaOrGt {
    tokens: TokenStream,
}

impl TokenStreamUntilCommaOrGt {
    pub fn tokens(&self) -> &TokenStream {
        &self.tokens
    }
}

impl Parse for TokenStreamUntilCommaOrGt {
    fn parse(input: &mut ParseBuffer) -> Result<Self> {
        let mut tokens = TokenStream::new();
        while let Some(token) = input.peek() {
            if matches!(
                token,
                TokenTree::Punct(punct)
                    if punct.as_char() == ',' || punct.as_char() == '>'
            ) {
                break;
            }
            tokens.extend(Some(input.next().expect("peeked token must exist")));
        }
        Ok(Self { tokens })
    }
}

impl ToTokens for TokenStreamUntilCommaOrGt {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.tokens.to_tokens(tokens);
    }
}
