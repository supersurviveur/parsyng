use parsyng::{Token, error::Result, parsyng};
use proc_macro::TokenStream;

struct Test {
    a: Token![match],
}

#[parsyng::proc_macro]
pub fn simple_macro(n: Token![&&]) -> Result<TokenStream> {
    eprintln!("{:?}", n);
    let tokens = parsyng! {
        let a = true #{ n } false;
        {
            let r#b = 0.3;
            r#b
        }
    };
    eprintln!("{}", tokens);
    // Err(Diagnostics::new_error("test"))
    Ok(tokens)
}
