use parsyng::{Token, combinator::{Cons, Punctuated}, error::Result, parsyng};

#[parsyng::proc_macro]
pub fn simple_macro(n: Cons<Punctuated<Token![match], Token![,]>, u8>) -> Result<u8> {
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
    Ok(0)
}
