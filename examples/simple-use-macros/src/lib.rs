use parsyng::{Parse, ToTokens, ast::item::Item, error::Result, parsyng};
use proc_macro::TokenStream;

#[derive(Parse, ToTokens)]
pub(crate) struct Foo {
    bar: u8,
}

#[parsyng::proc_macro(debug)]
pub fn simple_macro(n: (Item, Foo)) -> Result<TokenStream> {
    // eprintln!("{:#?}", n.2);
    let tokens = parsyng! {
        #{n.0}
        let a = true;
        {
            let r#b = 0.3;
            r#b
        }
    };
    // eprintln!("{}", tokens);
    Ok(tokens)
}
