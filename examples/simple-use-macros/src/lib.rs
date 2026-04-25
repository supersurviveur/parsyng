use parsyng::{
    Parse, ToTokens,
    ast::{crate_source::Crate, statements::Statement},
    error::Result,
    quote,
};
use proc_macro::TokenStream;

// #[derive(Parse, ToTokens)]
// pub(crate) struct Foo {
//     bar: u8,
// }

#[parsyng::proc_macro(debug)]
pub fn simple_macro(n: Crate) -> Result<TokenStream> {
    // eprintln!("{:#?}", n.2);
    let tokens = quote! {
        #{n}

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
