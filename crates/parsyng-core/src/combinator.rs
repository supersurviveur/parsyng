use std::slice::{Iter, IterMut};
use std::{marker::PhantomData, vec::IntoIter};

use crate::ToTokens;

use crate::error::Diagnostics;
use crate::parse::Invalid;
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
    fn to_tokens(&self, tokens: &mut crate::proc_macro::TokenStream) {
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

#[derive(Clone, Default, Debug)]
pub struct PunctuatedIter<'a, T, P> {
    content: Iter<'a, (T, P)>,
    last: Option<&'a T>,
}

#[derive(Default, Debug)]
pub struct PunctuatedIterMut<'a, T, P> {
    content: IterMut<'a, (T, P)>,
    last: Option<&'a mut T>,
}

impl<'a, T, P> Iterator for PunctuatedIter<'a, T, P> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.content.next() {
            Some(v) => Some(&v.0),
            None => self.last.take(),
        }
    }
}

impl<'a, T, P> Iterator for PunctuatedIterMut<'a, T, P> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.content.next() {
            Some(v) => Some(&mut v.0),
            None => self.last.take(),
        }
    }
}

impl<T, P, OnError> Punctuated<T, P, OnError> {
    pub const fn is_empty(&self) -> bool {
        self.content.is_empty() && self.last.is_none()
    }
    pub const fn len(&self) -> usize {
        self.content.len() + if self.last.is_some() { 1 } else { 0 }
    }
    pub fn push(&mut self, pair: (T, P)) {
        self.content.push(pair);
    }
    pub fn push_back(&mut self, pair: (T, P)) {
        self.content.insert(0, pair);
    }

    pub const fn trailing(&self) -> &Option<T> {
        &self.last
    }
    pub const fn new() -> Self {
        Self {
            content: Vec::new(),
            last: None,
            _phantom: PhantomData,
        }
    }
    pub const fn one(elem: T) -> Self {
        Self {
            content: Vec::new(),
            last: Some(elem),
            _phantom: PhantomData,
        }
    }

    pub fn iter_pairs(&self) -> Iter<'_, (T, P)> {
        self.content.iter()
    }

    pub fn iter(&self) -> PunctuatedIter<'_, T, P> {
        PunctuatedIter {
            content: self.content.iter(),
            last: self.last.as_ref(),
        }
    }

    pub fn iter_mut(&mut self) -> PunctuatedIterMut<'_, T, P> {
        PunctuatedIterMut {
            content: self.content.iter_mut(),
            last: self.last.as_mut(),
        }
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
    fn to_tokens(&self, tokens: &mut crate::proc_macro::TokenStream) {
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
pub struct GreedyVec<T> {
    inner: Vec<T>,
}
impl<T> GreedyVec<T> {
    pub fn inner(self) -> Vec<T> {
        self.inner
    }
}
impl<T: Parse> Parse for GreedyVec<T> {
    fn parse(input: &mut ParseBuffer) -> Result<Self> {
        let mut content = Vec::new();
        while !input.is_empty() {
            content.push(input.parse()?);
        }

        Ok(Self { inner: content })
    }
}

#[derive(Clone, Debug)]
pub enum Either<A, B, C = Invalid, D = Invalid, E = Invalid> {
    First(A),
    Second(B),
    Third(C),
    Fourth(D),
    Fifth(E),
}

impl<A: Parse, B: Parse, C: Parse, D: Parse, E: Parse> Parse for Either<A, B, C, D, E> {
    fn parse(input: &mut ParseBuffer) -> Result<Self> {
        let mut diagnostics = Diagnostics::empty();
        match input.try_parse() {
            Ok(first) => return Ok(Self::First(first)),
            Err(err) => diagnostics.join(err),
        }
        match input.try_parse() {
            Ok(second) => return Ok(Self::Second(second)),
            Err(err) => diagnostics.join(err),
        }
        match input.try_parse() {
            Ok(third) => return Ok(Self::Third(third)),
            Err(err) => diagnostics.join(err),
        }
        match input.try_parse() {
            Ok(fourth) => return Ok(Self::Fourth(fourth)),
            Err(err) => diagnostics.join(err),
        }
        match input.try_parse() {
            Ok(fifth) => return Ok(Self::Fifth(fifth)),
            Err(err) => diagnostics.join(err),
        }
        Err(diagnostics)
    }
}

impl<A: ToTokens, B: ToTokens, C: ToTokens, D: ToTokens, E: ToTokens> ToTokens
    for Either<A, B, C, D, E>
{
    fn to_tokens(&self, tokens: &mut crate::proc_macro::TokenStream) {
        match self {
            Either::First(first) => first.to_tokens(tokens),
            Either::Second(second) => second.to_tokens(tokens),
            Either::Third(third) => third.to_tokens(tokens),
            Either::Fourth(fourth) => fourth.to_tokens(tokens),
            Either::Fifth(fifth) => fifth.to_tokens(tokens),
        }
    }
}
