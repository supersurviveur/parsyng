use parsyng::{ast::{crate_source::Crate, item::r#struct::Struct}, error::Result, quote};
use proc_macro::TokenStream;

// #[derive(Parse, ToTokens)]
// pub(crate) struct Foo {
//     bar: u8,
// }

#[parsyng::proc_macro(debug)]
pub fn simple_macro(n: Crate) -> Result<TokenStream> {
    // eprintln!("{:#?}", n.2);
    let _tokens = quote! {
        #{n}

        let a = true;
        {
            let r#b = 0.3;
            r#b
        }
    };
    // println!("{}", _tokens);
    // Ok(_tokens)
    Ok(TokenStream::new())
    // Err(Diagnostics::new_error("{sen}"))
}

#[parsyng::proc_macro_attribute(debug)]
pub fn simple_macro_attribute(attrs: u8, _n: Crate) -> Result<TokenStream> {
    let _tokens = quote! {
        #{attrs}
    };
    println!("{}", _tokens);
    Ok(TokenStream::new())
}

#[parsyng::proc_macro_derive(Simple, debug)]
pub fn simple_macro_derive(n: Struct) -> Result<TokenStream> {
    let _tokens = quote! {
        #{n}
    };
    println!("{}", _tokens);
    Ok(TokenStream::new())
}
