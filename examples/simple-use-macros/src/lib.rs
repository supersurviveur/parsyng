use parsyng::parsyng;
// use proc_macro::TokenStream;

#[parsyng::proc_macro]
pub fn simple_macro(n: u8) -> u8 {
    let tokens = parsyng! {
        let a = #{ n.sub(3) };
        {
            let r#b = 0.3;
            r#b
        }
    };
    eprintln!("{}", tokens);
    9
}
