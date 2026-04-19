use crate::{
    error::{Diagnostics, Result},
    parse::{Parse, ParseBuffer},
    proc_macro::{Delimiter, Group, Span, TokenTree},
};

use std::ops::{Deref, DerefMut};

macro_rules! make_delimiters {
    ($($name:ident $l:literal $r:literal $delimiter:ident,)*) => {
        $(
        pub struct $name<T> {
            group: Group,
            content: T,
        }

        impl<T> $name<T> {
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
        )*
    };
}

make_delimiters! {
    Bracketed '[' ']' Bracket,
    Braced '{' '}' Brace,
    Parenthesized '(' ')' Parenthesis,
}
