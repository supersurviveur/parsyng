use parsyng::{Token, ast::item::ItemStruct, combinator::Punctuated, error::Result, parsyng};

#[parsyng::proc_macro]
pub fn simple_macro(n: (Punctuated<Token![match], Token![,]>, u8, ItemStruct)) -> Result<u8> {
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
