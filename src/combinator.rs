use crate::{
    error::Result,
    parse::{Nothing, Parse, ParseBuffer, Peek},
};

#[derive(Clone, Default)]
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

#[derive(Clone, Default)]
pub struct Punctuated<T, P> {
    content: Vec<(T, P)>,
    last: Option<T>,
}

impl<T: Parse, P: Peek> Parse for Punctuated<T, P> {
    fn parse(input: &mut ParseBuffer) -> Result<Self> {
        loop {
            let element = input.parse::<T>()?;

            if let Ok(punct) = input.peek_token::<P>() {}
        }
        input.parse()
    }
}
