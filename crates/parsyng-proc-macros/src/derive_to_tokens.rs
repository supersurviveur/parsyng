use parsyng_core::proc_macro::TokenStream;
use parsyng_core::{ast::item::ItemStruct, error, parse};
use parsyng_quote::quote;

pub fn derive_to_tokens(input: TokenStream) -> error::Result<TokenStream> {
    let mut stream = parse::ParseBuffer::new(input);

    let struct_item = stream.parse::<ItemStruct>()?;

    let mut fields = vec![];
    if let Some(struct_fields) = struct_item.fields() {
        for field in struct_fields.clone() {
            fields.push(quote! {
                self.#{ field.ident() }.to_tokens(tokens);
            });
        }
    }

    Ok(quote! {
        impl #{ struct_item.generic_parameters() } ToTokens for #{ struct_item.ident() } #{ struct_item.generic_parameters() } {
            fn to_tokens(&self, tokens: &mut parsyng::proc_macro::TokenStream) {
                #fields
            }
        }
    })
}
