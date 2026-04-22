use std::{marker::PhantomData, vec::IntoIter};

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

impl<A: Parse, B: Parse, C: Parse, D: Parse> Parse for (A, B, C, D) {
    fn parse(input: &mut ParseBuffer) -> Result<Self> {
        Ok((
            input.parse()?,
            input.parse()?,
            input.parse()?,
            input.parse()?,
        ))
    }
}

impl<T: Peek> Parse for Option<T> {
    fn parse(input: &mut ParseBuffer) -> Result<Self> {
        Ok(input.parse().ok())
    }
}
impl<T: Peek> Peek for Option<T> {}

#[derive(Clone, Default, Debug)]
pub struct Greedy;
#[derive(Clone, Default, Debug)]
pub struct StopOnError;

#[derive(Clone, Default, Debug)]
pub struct Punctuated<T, P, OnError = Greedy> {
    content: Vec<(T, P)>,
    last: Option<T>,
    _phantom: PhantomData<OnError>,
}

#[derive(Clone, Default, Debug)]
pub struct PunctuatedIntoIter<T, P> {
    content: IntoIter<(T, P)>,
    last: Option<T>,
}

impl<T, P, OnError> IntoIterator for Punctuated<T, P, OnError> {
    type Item = T;

    type IntoIter = PunctuatedIntoIter<T, P>;

    fn into_iter(self) -> Self::IntoIter {
        PunctuatedIntoIter {
            content: self.content.into_iter(),
            last: self.last,
        }
    }
}

impl<T, P> Iterator for PunctuatedIntoIter<T, P> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.content.next() {
            Some(v) => Some(v.0),
            None => self.last.take(),
        }
    }
}

impl<T, P, OnError> Punctuated<T, P, OnError> {
    pub fn is_empty(&self) -> bool {
        self.content.is_empty() && self.last.is_none()
    }
}

impl<T: Parse, P: Peek> Parse for Punctuated<T, P, StopOnError> {
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

        Ok(Self {
            content,
            last,
            _phantom: PhantomData,
        })
    }
}

impl<T: Parse, P: Parse> Parse for Punctuated<T, P, Greedy> {
    fn parse(input: &mut ParseBuffer) -> Result<Self> {
        let mut content = Vec::new();
        let mut last = None;

        while !input.is_empty() {
            let element = input.parse::<T>()?;
            if !input.is_empty() {
                content.push((element, input.parse()?));
            } else {
                last = Some(element);
            }
        }

        Ok(Self {
            content,
            last,
            _phantom: PhantomData,
        })
    }
}

impl<T: ToTokens, P: ToTokens, OnError> ToTokens for Punctuated<T, P, OnError> {
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
