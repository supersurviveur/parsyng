use crate::{
    ast::{
        delimiter::Parenthesized,
        tokens::{Crate, In, Pub, SelfValue},
    },
    combinator::Cons,
};

pub enum Visibility {
    Public(Pub),
    Crate(Pub, Parenthesized<Crate>),
    SelfVis(Pub, Parenthesized<SelfValue>),
    PubIn(Pub, Parenthesized<Cons<In, In>>),
    Private,
}
