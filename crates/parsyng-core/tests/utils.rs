use parsyng_core::ToTokens;
use parsyng_core::parse::ParseBuffer;
use parsyng_core::proc_macro::TokenStream;

pub fn parse_exact<T: parsyng_core::Parse>(tokens: TokenStream) -> T {
    let mut input = ParseBuffer::new(tokens);
    let value = input.parse::<T>().unwrap();
    assert!(input.is_empty());
    value
}

pub fn check<T: parsyng_core::Parse + ToTokens>(tokens: TokenStream) -> T {
    let expected = tokens.to_string();
    let parsed = parse_exact::<T>(tokens);
    let mut out = TokenStream::new();
    parsed.to_tokens(&mut out);
    assert_eq!(out.to_string(), expected);
    parsed
}
