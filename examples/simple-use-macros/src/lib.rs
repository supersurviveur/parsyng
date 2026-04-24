use parsyng::{Parse, ToTokens, ast::item::Item, error::Result, quote};
use proc_macro::TokenStream;

#[derive(Parse, ToTokens)]
pub(crate) struct Foo {
    bar: u8,
}

#[parsyng::proc_macro(debug)]
pub fn simple_macro(n: (Item, Item, Foo)) -> Result<TokenStream> {
    // eprintln!("{:#?}", n.2);
    let tokens = quote! {
        #{n.0}

        #{n.1}
        let a = true;
        {
            let r#b = 0.3;
            r#b
        }
    };
    eprintln!("{}", tokens);
    // Ok(tokens)
    Ok(TokenStream::new())
}
