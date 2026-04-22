use parsyng_quote::{parsyng, proc_macro::TokenStream};

use crate::{ast::item::ItemStruct, bootstrap};

pub fn derive_to_tokens(input: TokenStream) -> bootstrap::error::Result<TokenStream> {
    let mut stream = bootstrap::parse::ParseBuffer::new(input);

    let struct_item = stream.parse::<ItemStruct>()?;

    let mut fields = vec![];
    if let Some(struct_fields) = struct_item.fields() {
        for field in struct_fields.clone() {
            fields.push(parsyng! {
                self.#{ field.ident() }.to_tokens(tokens);
            });
        }
    }

    Ok(parsyng! {
        impl #{ struct_item.generic_parameters() } ToTokens for #{ struct_item.ident() } #{ struct_item.generic_parameters() } {
            fn to_tokens(&self, tokens: &mut parsyng::proc_macro::TokenStream) {
                #fields
            }
        }
    })
}
