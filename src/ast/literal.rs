use core::range::Range;

use crate::error::{Diagnostics, Result};
use crate::parse::Parse;

pub struct LiteralInt {
    content: String,
    prefix: Range<usize>,
    suffix: Range<usize>,
}

pub struct LiteralFloat {
    content: String,
    prefix: Range<usize>,
    suffix: Range<usize>,
}

pub enum Literal {
    Int(LiteralInt),
    Float(LiteralFloat),
}

macro_rules! integer_impls {
    ($($ty:ty,)*) => {
        $(impl Parse for $ty {
            fn parse(input: &mut crate::parse::ParseBuffer) -> crate::error::Result<Self> {
                if let Some(literal) = input.literal() {
                    let literal = literal.to_string();

                    if let Some(binary) = literal.strip_prefix("0b") {
                        if let Ok(parsed) = <$ty>::from_str_radix(binary, 2) {
                            return Ok(parsed);
                        }
                    }
                    if let Some(octal) = literal.strip_prefix("0o") {
                        if let Ok(parsed) = <$ty>::from_str_radix(octal, 8) {
                            return Ok(parsed);
                        }
                    }
                    if let Some(hex) = literal.strip_prefix("0x") {
                        if let Ok(parsed) = <$ty>::from_str_radix(hex, 16) {
                            return Ok(parsed);
                        }
                    }

                    if let Ok(parsed) = literal.parse() {
                        return Ok(parsed);
                    }
                }
                Err(Diagnostics::new_error("Expected literal"))
            }
        })*
    };
}

integer_impls! {
    u8, u16, u32, u64, u128, usize,
    i8, i16, i32, i64, i128, isize,
}
impl Parse for Literal {
    fn parse(input: &mut crate::parse::ParseBuffer) -> Result<Self> {
        if let Some(literal) = input.literal() {
            let literal_str = literal.to_string();

            if literal_str.starts_with('"') {
                todo!()
            } else if literal_str.starts_with('\'') {
                todo!()
            } else if literal_str.starts_with("b\"") {
                todo!()
            } else if literal_str.starts_with("b'") {
                todo!()
            } else if literal_str.contains('.') || literal_str.contains('e') {
                todo!()
            } else if literal_str
                .chars()
                .next()
                .is_some_and(|c| c.is_ascii_digit())
            {
                return parse_literal_int(literal_str, literal);
            }
        }
        Err(Diagnostics::new_error("Expected literal"))
    }
}

fn byte(bytes: &[u8], position: usize) -> u8 {
    bytes.get(position).copied().unwrap_or(0)
}

fn parse_literal_int(literal_str: String, literal: crate::proc_macro::Literal) -> Result<Literal> {
    let mut is_int = true;
    let bytes = literal_str.as_bytes();
    let bytes_iterator = bytes.iter();
    let len = literal_str.len();

    let radix;
    let prefix;
    match (byte(bytes, 0), byte(bytes, 1)) {
        (b'0', b'b') => {
            radix = 2;
            prefix = 0..2;
        }
        _ => {
            radix = 10;
            prefix = 0..0;
        }
    }

    // Skip prefix
    let bytes_iterator = bytes_iterator.skip(prefix.len());

    let mut position = prefix.len();

    for byte in bytes_iterator {
        position += 1;
        match byte {
            c if radix == 16 && c.is_ascii_hexdigit() => {}
            c if radix == 10 && c.is_ascii_digit() => {}
            c if radix == 8 && matches!(c, b'0'..=b'7') => {}
            c if radix == 2 && matches!(c, b'0'..=b'1') => {}
            b'.' => {
                is_int = false;
            }
            b'e' => {
                is_int = false;
            }
            _ => return Err(Diagnostics::new_error("Expected literal")),
        }
    }

    let suffix = position..len;

    Ok(if is_int {
        Literal::Int(LiteralInt {
            content: literal_str,
            prefix: prefix.into(),
            suffix: suffix.into(),
        })
    } else {
        Literal::Float(LiteralFloat {
            content: literal_str,
            prefix: prefix.into(),
            suffix: suffix.into(),
        })
    })
}
