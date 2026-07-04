use parsyng_core::ast::item::r#struct::StructFields;
use parsyng_core as parsyng;

use parsyng_core::proc_macro::TokenStream;
use parsyng_core::quote;
use parsyng_core::{ast::item::ItemStruct, error, parse};

pub fn derive_to_tokens(input: TokenStream) -> error::Result<TokenStream> {
    let mut stream = parse::ParseBuffer::new(input);

    let struct_item = stream.parse::<ItemStruct>()?;

    let mut fields = vec![];
    match &struct_item.fields {
        StructFields::Named(struct_fields) => {
            for field in struct_fields.iter() {
                fields.push(quote! {
                    self.#{ field.ident() }.to_tokens(tokens);
                });
            }
        }
        StructFields::Unnamed(_) => todo!(),
        StructFields::Unit => todo!(),
    }

    Ok(quote! {
        impl #{ struct_item.generic_parameters() } ToTokens for #{ struct_item.ident() } #{ struct_item.generic_parameters() } {
            fn to_tokens(&self, tokens: &mut parsyng::proc_macro::TokenStream) {
                #fields
            }
        }
    })
}
