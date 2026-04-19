#![allow(unexpected_cfgs)]

use parsyng_quote::ToTokens;

use crate::{
    error::{Diagnostics, Result},
    parse::{Parse, ParseBuffer},
    proc_macro::{Ident, Punct, Spacing, Span},
};

fn parse_keyword(input: &mut ParseBuffer, keyword: &str) -> Result<Ident> {
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
    (@keywords $($keyword:ident $i:literal => $keyword_name:ident)* @puncts $($punct:tt $($lit:literal),* => $punct_name:ident #[doc = $punct_usage:literal])*) => {
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
            $($keyword $i => $keyword_name)*
        }

        make_puncts! {
            $($punct $($lit)* => $punct_name #[doc = $punct_usage])*
        }
    };
}
macro_rules! make_puncts {
    (@type $a:literal) => {

        RustPunct1<$a>
    };
    (@type $a:literal $b:literal) => {

        RustPunct2<$a, $b>
    };
    (@type $a:literal $b:literal $c:literal) => {

        RustPunct3<$a, $b, $c>
    };
    ($($t:tt $($lit:literal)* => $name:ident #[doc = $usage:literal])*) => {
        $(
            #[doc = $usage]
            pub type $name = make_puncts!(@type $($lit)*);
        )*
    };
}
macro_rules! make_keywords {
    ($($keyword:tt $i:literal => $name:ident)*) => {
        const KEYWORDS: [&'static str; 53] = [$(stringify!($keyword)),*];

        $(
            #[doc = concat!("`", stringify!($keyword), "` keyword")]
            pub type $name = RustKeyword<$i>;
        )*
    };
}

make_tokens! {
    @keywords

    abstract 0  => Abstract
    as       1  => As
    async    2  => Async
    auto     3  => Auto
    await    4  => Await
    become   5  => Become
    box      6  => Box
    break    7  => Break
    const    8  => Const
    continue 9  => Continue
    crate    10 => Crate
    default  11 => Default
    do       12 => Do
    dyn      13 => Dyn
    else     14 => Else
    enum     15 => Enum
    extern   16 => Extern
    final    17 => Final
    fn       18 => Fn
    for      19 => For
    if       20 => If
    impl     21 => Impl
    in       22 => In
    let      23 => Let
    loop     24 => Loop
    macro    25 => Macro
    match    26 => Match
    mod      27 => Mod
    move     28 => Move
    mut      29 => Mut
    override 30 => Override
    priv     31 => Priv
    pub      32 => Pub
    raw      33 => Raw
    ref      34 => Ref
    return   35 => Return
    Self     36 => SelfType
    self     37 => SelfValue
    static   38 => Static
    struct   39 => Struct
    super    40 => Super
    trait    41 => Trait
    try      42 => Try
    type     43 => Type
    typeof   44 => Typeof
    union    45 => Union
    unsafe   46 => Unsafe
    unsized  47 => Unsized
    use      48 => Use
    virtual  49 => Virtual
    where    50 => Where
    while    51 => While
    yield    52 => Yield

    @puncts

    &    '&'           => And        /// bitwise and logical AND, borrow, references, reference patterns
    &&   '&', '&'      => AndAnd     /// lazy AND, borrow, references, reference patterns
    &=   '&', '='      => AndEq      /// bitwise AND assignment
    @    '@'           => At         /// subpattern binding
    ^    '^'           => Caret      /// bitwise and logical XOR
    ^=   '^', '='      => CaretEq    /// bitwise XOR assignment
    :    ':'           => Colon      /// various separators
    ,    ','           => Comma      /// various separators
    $    '$'           => Dollar     /// macros
    .    '.'           => Dot        /// field access, tuple index
    ..   '.', '.'      => DotDot     /// range, struct expressions, patterns, range patterns
    ...  '.', '.', '.' => DotDotDot  /// variadic functions, range patterns
    ..=  '.', '.', '=' => DotDotEq   /// inclusive range, range patterns
    =    '='           => Eq         /// assignment, attributes, various type definitions
    ==   '=', '='      => EqEq       /// equal
    =>   '=', '>'      => FatArrow   /// match arms, macros
    >=   '>', '='      => Ge         /// greater than or equal to, generics
    >    '>'           => Gt         /// greater than, generics, paths
    <-   '<', '-'      => LArrow     /// unused
    <=   '<', '='      => Le         /// less than or equal to
    <    '<'           => Lt         /// less than, generics, paths
    -    '-'           => Minus      /// subtraction, negation
    -=   '-', '='      => MinusEq    /// subtraction assignment
    !=   '!', '='      => Ne         /// not equal
     !    '!'          => Not        /// bitwise and logical NOT, macro calls, inner attributes, never type, negative impls
    |    '|'           => Or         /// bitwise and logical OR, closures, patterns in match, if let, and while let
    |=   '|', '='      => OrEq       /// bitwise OR assignment
    ||   '|', '|'      => OrOr       /// lazy OR, closures
    ::   ':', ':'      => PathSep    /// path separator
    %    '%'           => Percent    /// remainder
    %=   '%', '='      => PercentEq  /// remainder assignment
    +    '+'           => Plus       /// addition, trait bounds, macro Kleene matcher
    +=   '+', '='      => PlusEq     /// addition assignment
    #    '#'           => Pound      /// attributes
    ?    '?'           => Question   /// question mark operator, questionably sized, macro Kleene matcher
    ->   '-', '>'      => RArrow     /// function return type, closure return type, function pointer type
    ;    ';'           => Semi       /// terminator for various items and statements, array types
    <<   '<', '<'      => Shl        /// shift left, nested generics
    <<=  '<', '<', '=' => ShlEq      /// shift left assignment
    >>   '>', '>'      => Shr        /// shift right, nested generics
    >>=  '>', '>', '=' => ShrEq      /// shift right assignment, nested generics
    /    '/'           => Slash      /// division
    /=   '/', '='      => SlashEq    /// division assignment
    *    '*'           => Star       /// multiplication, dereference, raw pointers, macro Kleene matcher, use wildcards
    *=   '*', '='      => StarEq     /// multiplication assignment
    ~    '~'           => Tilde      /// unused since before Rust 1.0
}

#[derive(Debug, Clone)]
pub struct RustKeyword<const K: u8> {
    ident: Ident,
}

impl<const K: u8> Parse for RustKeyword<K> {
    fn parse(input: &mut ParseBuffer) -> Result<Self> {
        Ok(Self {
            ident: parse_keyword(input, KEYWORDS[K as usize])?,
        })
    }
}
impl<const K: u8> ToTokens for RustKeyword<K> {
    fn to_tokens(&self, tokens: &mut parsyng_quote::proc_macro::TokenStream) {
        tokens.extend(Some(self.ident.clone()));
    }
}

#[derive(Debug, Clone)]
pub struct RustPunct1<const A: char>([Punct; 1]);

#[derive(Debug, Clone)]
pub struct RustPunct2<const A: char, const B: char>([Punct; 2]);

#[derive(Debug, Clone)]
pub struct RustPunct3<const A: char, const B: char, const C: char>([Punct; 3]);

impl<const A: char> RustPunct1<A> {
    pub fn span(&self) -> Span {
        self.0[0].span()
    }
    pub fn spans(&self) -> [Span; 1] {
        self.0.clone().map(|punct| punct.span())
    }
}

impl<const A: char, const B: char> RustPunct2<A, B> {
    pub fn spans(&self) -> [Span; 2] {
        self.0.clone().map(|punct| punct.span())
    }
}

impl<const A: char, const B: char, const C: char> RustPunct3<A, B, C> {
    pub fn spans(&self) -> [Span; 3] {
        self.0.clone().map(|punct| punct.span())
    }
}

impl<const A: char> Parse for RustPunct1<A> {
    fn parse(input: &mut ParseBuffer) -> Result<Self> {
        let error_span: Span = input.span();
        if let Some(punct1) = input.punct()
            && punct1.as_char() == A
        {
            return Ok(Self([punct1]));
        }
        Err(Diagnostics::new_error_spanned(
            format!("Expected token `{}`", A),
            error_span,
        ))
    }
}

impl<const A: char> ToTokens for RustPunct1<A> {
    fn to_tokens(&self, tokens: &mut parsyng_quote::proc_macro::TokenStream) {
        Punct::new(A, Spacing::Alone).to_tokens(tokens);
    }
}
impl<const A: char, const B: char> Parse for RustPunct2<A, B> {
    fn parse(input: &mut ParseBuffer) -> Result<Self> {
        let error_span: Span = input.span();
        if let Some(punct1) = input.punct()
            && punct1.as_char() == A
            && let Some(punct2) = input.punct()
            && punct2.as_char() == B
        {
            return Ok(Self([punct1, punct2]));
        }
        Err(Diagnostics::new_error_spanned(
            format!("Expected token `{}{}`", A, B),
            error_span,
        ))
    }
}

impl<const A: char, const B: char> ToTokens for RustPunct2<A, B> {
    fn to_tokens(&self, tokens: &mut parsyng_quote::proc_macro::TokenStream) {
        Punct::new(A, Spacing::Joint).to_tokens(tokens);
        Punct::new(B, Spacing::Alone).to_tokens(tokens);
    }
}

impl<const A: char, const B: char, const C: char> Parse for RustPunct3<A, B, C> {
    fn parse(input: &mut ParseBuffer) -> Result<Self> {
        let error_span: Span = input.span();
        if let Some(punct1) = input.punct()
            && punct1.as_char() == A
            && let Some(punct2) = input.punct()
            && punct2.as_char() == B
            && let Some(punct3) = input.punct()
            && punct3.as_char() == C
        {
            return Ok(Self([punct1, punct2, punct3]));
        }
        Err(Diagnostics::new_error_spanned(
            format!("Expected token `{}{}{}`", A, B, C),
            error_span,
        ))
    }
}

impl<const A: char, const B: char, const C: char> ToTokens for RustPunct3<A, B, C> {
    fn to_tokens(&self, tokens: &mut parsyng_quote::proc_macro::TokenStream) {
        Punct::new(A, Spacing::Joint).to_tokens(tokens);
        Punct::new(B, Spacing::Joint).to_tokens(tokens);
        Punct::new(C, Spacing::Alone).to_tokens(tokens);
    }
}
