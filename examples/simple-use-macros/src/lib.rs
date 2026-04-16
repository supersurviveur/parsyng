use proc_macro::TokenStream;
use parsyng::{parse::Parse, parsyng};

#[proc_macro]
pub fn simple_macro(input: TokenStream) -> TokenStream {
    let mut parse = parsyng::parse::ParseBuffer::new(input.clone());
    let n = u8::parse(&mut parse);
    eprintln!("{:?}", n);
    let tokens = parsyng! {
        let a = {{ n.unwrap() }};
        {
            let r#b = 0.3;
            r#b
        }
    };
    eprintln!("{}", tokens);
    input
}
