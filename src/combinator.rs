use parsyng_quote::ToTokens;

use crate::{
    error::Result,
    parse::{Nothing, Parse, ParseBuffer, Peek},
};

#[derive(Clone, Default, Debug)]
pub struct Cons<A, B, C = Nothing, D = Nothing, E = Nothing> {
    pub first: A,
    pub second: B,
    pub third: C,
    pub fourth: D,
    pub fifth: E,
}

impl<A: Parse, B: Parse, C: Parse, D: Parse, E: Parse> Parse for Cons<A, B, C, D, E> {
    fn parse(input: &mut ParseBuffer) -> Result<Self> {
        Ok(Self {
            first: input.parse()?,
            second: input.parse()?,
            third: input.parse()?,
            fourth: input.parse()?,
            fifth: input.parse()?,
        })
    }
}
impl<A: ToTokens, B: ToTokens, C: ToTokens, D: ToTokens, E: ToTokens> ToTokens
    for Cons<A, B, C, D, E>
{
    fn to_tokens(&self, tokens: &mut parsyng_quote::proc_macro::TokenStream) {
        self.first.to_tokens(tokens);
        self.second.to_tokens(tokens);
        self.third.to_tokens(tokens);
        self.fourth.to_tokens(tokens);
        self.fifth.to_tokens(tokens);
    }
}
impl<A: Parse, B: Parse> Parse for (A, B) {
    fn parse(input: &mut ParseBuffer) -> Result<Self> {
        Ok((input.parse()?, input.parse()?))
    }
}
impl<A: Parse, B: Parse, C: Parse> Parse for (A, B, C) {
    fn parse(input: &mut ParseBuffer) -> Result<Self> {
        Ok((input.parse()?, input.parse()?, input.parse()?))
    }
}

impl<T: Peek> Parse for Option<T> {
    fn parse(input: &mut ParseBuffer) -> Result<Self> {
        Ok(input.parse().ok())
    }
}
impl<T: Peek> Peek for Option<T> {}

#[derive(Clone, Default, Debug)]
pub struct Punctuated<T, P> {
    content: Vec<(T, P)>,
    last: Option<T>,
}

impl<T: Parse, P: Peek> Parse for Punctuated<T, P> {
    fn parse(input: &mut ParseBuffer) -> Result<Self> {
        let mut content = Vec::new();
        let mut last = None;
        while let Ok(element) = input.try_advance(|input| input.parse::<T>()) {
            if let Ok(punct) = input.peek_parse() {
                content.push((element, punct));
            } else {
                last = Some(element);
                break;
            }
        }

        Ok(Self { content, last })
    }
}
impl<T: ToTokens, P: ToTokens> ToTokens for Punctuated<T, P> {
    fn to_tokens(&self, tokens: &mut parsyng_quote::proc_macro::TokenStream) {
        for pair in &self.content {
            pair.0.to_tokens(tokens);
            pair.1.to_tokens(tokens);
        }
        self.last.to_tokens(tokens);
    }
}
impl<T: Parse> Parse for Vec<T> {
    fn parse(input: &mut ParseBuffer) -> Result<Self> {
        let mut content = Vec::new();
        while let Ok(element) = input.try_advance(|input| input.parse::<T>()) {
            content.push(element);
        }

        Ok(content)
    }
}
