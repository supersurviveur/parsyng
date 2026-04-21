use parsyng::{
    Parse, ToTokens, Token,
    ast::item::ItemStruct,
    combinator::{Punctuated, StopOnError},
    error::Result,
    parsyng,
};

#[derive(Parse, ToTokens)]
pub(crate) struct Foo {
    bar: u8,
}

#[parsyng::proc_macro]
pub fn simple_macro(
    n: (
        Punctuated<Token![match], Token![,], StopOnError>,
        u8,
        ItemStruct,
        Foo,
    ),
) -> Result<u8> {
    eprintln!("{:#?}", n.2);
    let tokens = parsyng! {
        let a = true #{ n } false;
        {
            let r#b = 0.3;
            r#b
        }
    };
    eprintln!("{}", tokens);
    // Err(Diagnostics::new_error("test"))
    Ok(0)
}
