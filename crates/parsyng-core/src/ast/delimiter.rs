use crate::{
    error::{Diagnostics, Result},
    parse::{Parse, ParseBuffer},
    proc_macro::{Delimiter, Group, Span, TokenStream},
};
use parsyng_quote::ToTokens;

use std::ops::{Deref, DerefMut};

macro_rules! make_delimiters {
    ($($name:ident $l:literal $r:literal $delimiter:ident,)*) => {
        $(

        #[derive(Clone, Debug)]
        pub struct $name<T> {
            group: Group,
            content: T,
        }

        impl<T> $name<T> {
            pub fn new(group: Group, content: T) -> Self {
                Self { group, content }
            }
            pub fn span(&self) -> Span {
                self.group.span()
            }
        }

        impl<T> Deref for $name<T> {
            type Target = T;

            fn deref(&self) -> &Self::Target {
                &self.content
            }
        }

        impl<T> DerefMut for $name<T> {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.content
            }
        }

        impl<T: Parse> Parse for $name<T> {
            fn parse(input: &mut ParseBuffer) -> Result<Self> {
                match input.group() {
                    Some(group) if group.delimiter() == Delimiter::$delimiter => {
                        let mut stream = ParseBuffer::new(group.stream());
                        let content = stream.parse::<T>()?;

                        if stream.is_empty() {
                            Ok(Self { group, content })
                        } else {
                            Err(Diagnostics::new_error_spanned(
                                concat!("Expected `", $r, "`"),
                                stream.span(),
                            ))
                        }
                    }
                    _ => Err(Diagnostics::new_error_spanned(
                        concat!("Expected `", $l, "`"),
                        input.span(),
                    )),
                }
            }
        }
        impl<T: ToTokens> ToTokens for $name<T> {
            fn to_tokens(&self, tokens: &mut parsyng_quote::proc_macro::TokenStream) {
                let mut inner_tokens = TokenStream::new();
                self.content.to_tokens(&mut inner_tokens);
                tokens.extend(Some(Group::new(Delimiter::$delimiter, inner_tokens)));
            }
        }
        )*
    };
}

make_delimiters! {
    Bracketed '[' ']' Bracket,
    Braced '{' '}' Brace,
    Parenthesized '(' ')' Parenthesis,
}
