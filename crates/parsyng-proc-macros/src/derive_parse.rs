use parsyng_quote::proc_macro::TokenStream;

use crate::bootstrap;

pub fn derive_parse(input: TokenStream) -> bootstrap::error::Result<TokenStream> {
    let mut stream = bootstrap::parse::ParseBuffer::new(input);

    stream.parse::<Token![pub]>()?;
    stream.parse::<Token![struct]>()?;
    todo!()
}
