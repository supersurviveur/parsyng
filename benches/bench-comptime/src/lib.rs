use proc_macro::TokenStream;

#[cfg(all(feature = "empty"))]
#[proc_macro]
pub fn macro_bench(_: TokenStream) -> TokenStream {
    TokenStream::new()
}
