use parsyng::{error::Result, parsyng};
// use proc_macro::TokenStream;

#[parsyng::proc_macro]
pub fn simple_macro(n: u8) -> Result<u8> {
    let tokens = parsyng! {
        let a = #{ n };
        {
            let r#b = 0.3;
            r#b
        }
    };
    eprintln!("{}", tokens);
    // Err(Diagnostics::new_error("test"))
    Ok(9)
}
