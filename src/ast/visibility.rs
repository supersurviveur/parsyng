use parsyng_quote::ToTokens;

use crate::{
    ast::{
        delimiter::Parenthesized,
        path::SimplePath,
        tokens::{Crate, In, Pub, SelfValue},
    },
    error::Diagnostics,
    parse::{Parse, ParseBuffer},
    proc_macro::Delimiter,
};

#[derive(Clone, Debug)]
pub enum Visibility {
    Public(Pub),
    Crate(Pub, Parenthesized<Crate>),
    SelfVis(Pub, Parenthesized<SelfValue>),
    PubIn(Pub, Parenthesized<(In, SimplePath)>),
    Private,
}

impl Parse for Visibility {
    fn parse(input: &mut crate::parse::ParseBuffer) -> crate::error::Result<Self> {
        if let Ok(pub_token) = input.peek_parse::<Pub>() {
            if let Some(group) = input.peek_group()
                && group.delimiter() == Delimiter::Parenthesis
            {
                let mut group_input = ParseBuffer::new(group.stream());
                if let Ok(crate_token) = group_input.peek_parse::<Crate>() {
                    Ok(Self::Crate(
                        pub_token,
                        Parenthesized::new(input.group().unwrap(), crate_token),
                    ))
                } else if let Ok(self_token) = group_input.peek_parse::<SelfValue>() {
                    Ok(Self::SelfVis(
                        pub_token,
                        Parenthesized::new(input.group().unwrap(), self_token),
                    ))
                } else if let Ok(in_token) = group_input.peek_parse::<In>() {
                    let path = group_input.parse()?;
                    Ok(Self::PubIn(
                        pub_token,
                        Parenthesized::new(input.group().unwrap(), (in_token, path)),
                    ))
                } else {
                    Err(Diagnostics::new_error_spanned(
                        "Expected `in`, `crate` or `self`",
                        input.span(),
                    ))
                }
            } else {
                Ok(Self::Public(pub_token))
            }
        } else {
            Ok(Self::Private)
        }
    }
}
impl ToTokens for Visibility {
    fn to_tokens(&self, tokens: &mut parsyng_quote::proc_macro::TokenStream) {
        match self {
            Visibility::Public(pub_keyword) => pub_keyword.to_tokens(tokens),
            Visibility::Crate(rust_keyword, parenthesized) => {
                rust_keyword.to_tokens(tokens);
                parenthesized.to_tokens(tokens);
            }
            Visibility::SelfVis(rust_keyword, parenthesized) => {
                rust_keyword.to_tokens(tokens);
                parenthesized.to_tokens(tokens);
            }
            Visibility::PubIn(rust_keyword, parenthesized) => {
                rust_keyword.to_tokens(tokens);
                parenthesized.to_tokens(tokens);
            }
            Visibility::Private => {}
        }
    }
}
