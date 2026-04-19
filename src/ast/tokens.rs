use core::mem::MaybeUninit;

use parsyng_quote::ToTokens;

use crate::{
    error::{Diagnostics, Result},
    parse::Parse,
    proc_macro::{Ident, Punct, Span},
};

fn parse_puncts<const N: usize>(
    input: &mut crate::parse::ParseBuffer,
    token: &str,
) -> Result<[Punct; N]> {
    let mut result = MaybeUninit::uninit();
    let error_span: Span = input.span();
    for (i, c) in token.as_bytes().iter().enumerate() {
        match input.punct() {
            Some(punct) => {
                if punct.as_char() != *c as char {
                    break;
                }

                <MaybeUninit<_> as AsMut<[_; _]>>::as_mut(&mut result)[i].write(punct);

                if i == token.len() - 1 {
                    return Ok(unsafe { result.assume_init() });
                }
            }
            None => break,
        }
    }
    Err(Diagnostics::new_error_spanned(
        format!("Expected token `{}`", token),
        error_span,
    ))
}

fn parse_keyword(input: &mut crate::parse::ParseBuffer, keyword: &str) -> Result<Ident> {
    let span = input.span();
    let mk_error =
        || Diagnostics::new_error_spanned(format!("Expected keyword `{}`", keyword), span);

    input.ident().ok_or_else(mk_error).and_then(|ident| {
        #[allow(clippy::cmp_owned)]
        if ident.to_string() == keyword {
            Ok(ident)
        } else {
            Err(mk_error())
        }
    })
}

macro_rules! make_tokens {
    (@keywords $($keyword:ident => $keyword_name:ident)* @puncts $($punct:tt => $punct_name:ident #[doc = $punct_usage:literal])*) => {
        #[allow(unexpected_cfgs)]
        #[cfg_attr(not(feature = "bootstrap"), macro_export)]
        macro_rules! Token {
            $(
                ($keyword) => {
                    parsyng::ast::tokens::$keyword_name
                };
            )*
            $(
                ($punct) => {
                    parsyng::ast::tokens::$punct_name
                };
            )*
        }

        make_keywords! {
            $($keyword => $keyword_name)*
        }

        make_puncts! {
            $($punct => $punct_name #[doc = $punct_usage])*
        }
    };
}
macro_rules! make_puncts {
    ($($t:tt => $name:ident #[doc = $usage:literal])*) => {
        $(
            #[derive(Debug, Clone)]
            #[doc = $usage]
            pub struct $name {
                puncts: [Punct; stringify!($t).len()],
            }

            impl $name {
                pub fn puncts(&self) -> &[Punct; stringify!($t).len()] {
                    &self.puncts
                }
                pub fn spans(&self) -> [Span; stringify!($t).len()] {
                    self.puncts.clone().map(|punct| punct.span())
                }
            }

            impl Parse for $name {
                fn parse(input: &mut crate::parse::ParseBuffer) -> crate::error::Result<Self> {
                    Ok(Self {
                        puncts: parse_puncts(input, stringify!($t))?,
                    })
                }
            }

            impl ToTokens for $name {
                fn to_tokens(&self, tokens: &mut parsyng_quote::proc_macro::TokenStream) {
                    for (i, c) in stringify!($t).as_bytes().iter().enumerate() {
                        tokens.extend(Some(crate::proc_macro::Punct::new(*c as char, if i != stringify!($t).len() - 1 {
                            crate::proc_macro::Spacing::Joint
                        } else {
                            crate::proc_macro::Spacing::Alone
                        })));
                    }
                }
            }
        )*
    };
}
macro_rules! make_keywords {
    ($($keyword:tt => $name:ident)*) => {
        $(
            #[derive(Debug, Clone)]
            #[doc = concat!("`", stringify!($keyword), "` keyword")]
            pub struct $name {
                ident: Ident,
            }

            impl $name {
                pub fn to_ident(self) -> Ident {
                    self.ident
                }
                pub fn as_ident(&self) -> &Ident {
                    &self.ident
                }
                pub fn span(&self) -> Span {
                    self.ident.span()
                }
            }

            impl Parse for $name {
                fn parse(input: &mut crate::parse::ParseBuffer) -> crate::error::Result<Self> {
                    Ok(Self {
                        ident: parse_keyword(input, stringify!($keyword))?,
                    })
                }
            }

            impl ToTokens for $name {
                fn to_tokens(&self, tokens: &mut parsyng_quote::proc_macro::TokenStream) {
                    tokens.extend(Some(self.ident.clone()));
                }
            }
        )*
    };
}

make_tokens! {
    @keywords

    abstract => Abstract
    as       => As
    async    => Async
    auto     => Auto
    await    => Await
    become   => Become
    box      => Box
    break    => Break
    const    => Const
    continue => Continue
    crate    => Crate
    default  => Default
    do       => Do
    dyn      => Dyn
    else     => Else
    enum     => Enum
    extern   => Extern
    final    => Final
    fn       => Fn
    for      => For
    if       => If
    impl     => Impl
    in       => In
    let      => Let
    loop     => Loop
    macro    => Macro
    match    => Match
    mod      => Mod
    move     => Move
    mut      => Mut
    override => Override
    priv     => Priv
    pub      => Pub
    raw      => Raw
    ref      => Ref
    return   => Return
    Self     => SelfType
    self     => SelfValue
    static   => Static
    struct   => Struct
    super    => Super
    trait    => Trait
    try      => Try
    type     => Type
    typeof   => Typeof
    union    => Union
    unsafe   => Unsafe
    unsized  => Unsized
    use      => Use
    virtual  => Virtual
    where    => Where
    while    => While
    yield    => Yield

    @puncts

    &        => And        /// bitwise and logical AND, borrow, references, reference patterns
    &&       => AndAnd     /// lazy AND, borrow, references, reference patterns
    &=       => AndEq      /// bitwise AND assignment
    @        => At         /// subpattern binding
    ^        => Caret      /// bitwise and logical XOR
    ^=       => CaretEq    /// bitwise XOR assignment
    :        => Colon      /// various separators
    ,        => Comma      /// various separators
    $        => Dollar     /// macros
    .        => Dot        /// field access, tuple index
    ..       => DotDot     /// range, struct expressions, patterns, range patterns
    ...      => DotDotDot  /// variadic functions, range patterns
    ..=      => DotDotEq   /// inclusive range, range patterns
    =        => Eq         /// assignment, attributes, various type definitions
    ==       => EqEq       /// equal
    =>       => FatArrow   /// match arms, macros
    >=       => Ge         /// greater than or equal to, generics
    >        => Gt         /// greater than, generics, paths
    <-       => LArrow     /// unused
    <=       => Le         /// less than or equal to
    <        => Lt         /// less than, generics, paths
    -        => Minus      /// subtraction, negation
    -=       => MinusEq    /// subtraction assignment
    !=       => Ne         /// not equal
    !        => Not        /// bitwise and logical NOT, macro calls, inner attributes, never type, negative impls
    |        => Or         /// bitwise and logical OR, closures, patterns in match, if let, and while let
    |=       => OrEq       /// bitwise OR assignment
    ||       => OrOr       /// lazy OR, closures
    ::       => PathSep    /// path separator
    %        => Percent    /// remainder
    %=       => PercentEq  /// remainder assignment
    +        => Plus       /// addition, trait bounds, macro Kleene matcher
    +=       => PlusEq     /// addition assignment
    #        => Pound      /// attributes
    ?        => Question   /// question mark operator, questionably sized, macro Kleene matcher
    ->       => RArrow     /// function return type, closure return type, function pointer type
    ;        => Semi       /// terminator for various items and statements, array types
    <<       => Shl        /// shift left, nested generics
    <<=      => ShlEq      /// shift left assignment
    >>       => Shr        /// shift right, nested generics
    >>=      => ShrEq      /// shift right assignment, nested generics
    /        => Slash      /// division
    /=       => SlashEq    /// division assignment
    *        => Star       /// multiplication, dereference, raw pointers, macro Kleene matcher, use wildcards
    *=       => StarEq     /// multiplication assignment
    ~        => Tilde      /// unused since before Rust 1.0
}
