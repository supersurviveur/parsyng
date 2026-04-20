use core::range::Range;

use crate::{
    ast::identifiers::is_identifier_or_keyword,
    error::{Diagnostics, Result},
    parse::{Parse, ParseBuffer},
    proc_macro::Span,
};

#[derive(Debug, Clone)]
pub struct LiteralNumber {
    content: String,
    prefix: Range<usize>,
    suffix: Range<usize>,
    span: Span,
}

#[derive(Debug, Clone)]
pub struct LiteralFloat {
    content: String,
    suffix: Range<usize>,
    span: Span,
}

#[derive(Debug, Clone)]
pub enum Literal {
    UInt(LiteralNumber),
    Float(LiteralFloat),
}

impl LiteralNumber {
    pub fn content(&self) -> &str {
        &self.content[self.prefix.end..self.suffix.start]
    }
    pub fn prefix(&self) -> &str {
        &self.content[self.prefix]
    }
    pub fn suffix(&self) -> &str {
        &self.content[self.suffix]
    }
    pub fn span(&self) -> Span {
        self.span
    }
}
impl LiteralFloat {
    pub fn content(&self) -> &str {
        &self.content[0..self.suffix.start]
    }
    pub fn suffix(&self) -> &str {
        &self.content[self.suffix]
    }
    pub fn span(&self) -> Span {
        self.span
    }
}

macro_rules! unsigned_integer_impls {
    ($($ty:ty,)*) => {
        $(impl Parse for $ty {
            fn parse(input: &mut ParseBuffer) -> Result<Self> {
                input.parse::<LiteralNumber>().and_then(|lit| {
                    if !lit.suffix().is_empty() && lit.suffix() != stringify!($ty) {
                        return Err(Diagnostics::new_error_spanned(format!(concat!("Expected ", stringify!($ty), ", found `{}`"), lit.suffix()), lit.span()));
                    }
                    lit.content().parse::<$ty>().map_err(|err| {
                        Diagnostics::new_error_spanned(format!("Failed to parse literal: {}", err), lit.span())
                    })
                })
            }
        })*
    };
}

unsigned_integer_impls! {
    u8, u16, u32, u64, u128, usize,
}

impl Parse for Literal {
    fn parse(input: &mut ParseBuffer) -> Result<Self> {
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
            } else {
                if (literal_str.starts_with("0x") || !literal_str.contains(['f', 'e']))
                    && !literal_str.contains('.')
                {
                    return Ok(Literal::UInt(parse_integer_literal(
                        literal_str,
                        literal.span(),
                    )?));
                } else if let Ok(float) = parse_float_literal(literal_str, literal.span()) {
                    return Ok(Literal::Float(float));
                }
            }
        }
        Err(Diagnostics::new_error("Expected literal"))
    }
}

impl Parse for LiteralNumber {
    fn parse(input: &mut ParseBuffer) -> Result<Self> {
        if let Some(literal) = input.literal() {
            let literal_str = literal.to_string();
            return parse_integer_literal(literal_str, literal.span());
        }
        Err(Diagnostics::new_error("Expected number literal"))
    }
}

impl Parse for LiteralFloat {
    fn parse(input: &mut ParseBuffer) -> Result<Self> {
        if let Some(literal) = input.literal() {
            let literal_str = literal.to_string();
            return parse_float_literal(literal_str, literal.span());
        }
        Err(Diagnostics::new_error("Expected float literal"))
    }
}

fn byte(bytes: &str, position: usize) -> u8 {
    bytes.as_bytes().get(position).copied().unwrap_or(0)
}

fn parse_integer_literal(literal: String, span: Span) -> Result<LiteralNumber> {
    let s = literal.as_str();
    let len = literal.len();

    let radix;
    let prefix;
    match (byte(s, 0), byte(s, 1)) {
        (b'0', b'b') => {
            radix = 2;
            prefix = 0..2;
        }
        (b'0', b'x') => {
            radix = 16;
            prefix = 0..2;
        }
        (b'0', b'o') => {
            radix = 8;
            prefix = 0..2;
        }
        _ => {
            radix = 10;
            prefix = 0..0;
        }
    }

    let mut position = prefix.len();
    let mut has_digit = false;

    for byte in &s.as_bytes()[position..] {
        match byte {
            c if radix == 16 && c.is_ascii_hexdigit() => {}
            c if radix == 10 && c.is_ascii_digit() => {}
            c if radix == 8 && matches!(c, b'0'..=b'7') => {}
            c if radix == 2 && matches!(c, b'0'..=b'1') => {}
            b'_' => {
                if !has_digit {
                    return Err(Diagnostics::new_error_spanned(
                        "Expected a digit, found `_`",
                        span,
                    ));
                }
            }
            _ => break,
        }
        has_digit = true;
        position += 1;
    }

    let suffix = Range::from(position..len);

    if !suffix.is_empty() && !is_identifier_or_keyword(&s[suffix]) {
        return Err(Diagnostics::new_error_spanned(
            "Expected identifier as integer suffix",
            span,
        ));
    }

    Ok(LiteralNumber {
        content: literal,
        prefix: prefix.into(),
        suffix,
        span,
    })
}

fn parse_float_exponent(s: &str, span: Span) -> Result<usize> {
    let mut position = 0;
    match byte(s, 0) {
        b'e' | b'E' => {
            position += 1;
        }
        _ => {
            return Err(Diagnostics::new_error_spanned(
                "Expected `e` or `E` at the beginning of a float exponent",
                span,
            ));
        }
    }
    match byte(s, 1) {
        b'+' | b'-' => {
            position += 1;
        }
        _ => {}
    }

    let mut has_digit = false;

    for byte in &s.as_bytes()[position..] {
        match byte {
            b'_' => {}
            c if c.is_ascii_digit() => {
                has_digit = true;
            }
            _ => break,
        }
        position += 1;
    }

    if !has_digit {
        return Err(Diagnostics::new_error_spanned(
            "Expected at least one digit after exponent",
            span,
        ));
    }

    Ok(position)
}

fn parse_float_literal(literal: String, span: Span) -> Result<LiteralFloat> {
    let s = literal.as_str();
    let len = literal.len();

    let mut position = 0;

    let mut has_digit = false;
    let mut has_point = false;

    for byte in s.as_bytes() {
        // byte next a `.`
        if has_point && !has_digit && unicode_ident::is_xid_start(*byte as char) {
            return Err(Diagnostics::new_error_spanned(
                format!("Unexpected `{}` after `.` in float literal", *byte as char),
                span,
            ));
        }
        match byte {
            c if c.is_ascii_digit() => {
                position += 1;
                has_digit = true;
            }
            b'.' => {
                if has_point {
                    return Err(Diagnostics::new_error_spanned("Unexpected `.`", span));
                }
                has_point = true;
                position += 1;
                has_digit = false;
            }
            b'e' | b'E' => {
                if !has_digit {
                    return Err(Diagnostics::new_error_spanned(
                        "Expected a digit, found `e`",
                        span,
                    ));
                }
                position += parse_float_exponent(&s[position..], span)?;
                break;
            }
            b'_' => {
                if !has_digit {
                    return Err(Diagnostics::new_error_spanned(
                        "Expected a digit, found `_`",
                        span,
                    ));
                }
                position += 1;
            }
            _ => break,
        }
    }

    let suffix = Range::from(position..len);

    if !suffix.is_empty() && !is_identifier_or_keyword(&s[suffix]) {
        return Err(Diagnostics::new_error_spanned(
            "Expected identifier as float suffix",
            span,
        ));
    }

    Ok(LiteralFloat {
        content: literal,
        suffix,
        span,
    })
}
