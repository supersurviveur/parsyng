use parsyng::quote;

// Add the `debug` attribute to get debug informations even if rust can't compile the macro output.
// #[parsyng::proc_macro(debug)]
#[parsyng::proc_macro]
pub fn faulty_macro(_input: ()) -> proc_macro::TokenStream {
    // Some quote output returning an invalid code due to the star
    quote! {
        let foo = 9 - 3 *;
    }
}
