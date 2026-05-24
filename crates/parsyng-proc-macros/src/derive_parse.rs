use parsyng_core::{ast::item::ItemStruct, error, parse};
use parsyng_quote::quote;
use proc_macro::TokenStream;

pub fn derive_parse(input: TokenStream) -> error::Result<TokenStream> {
    let mut stream = parse::ParseBuffer::new(input);

    let struct_item = stream.parse::<ItemStruct>()?;

    let mut fields = vec![];
    if let Some(struct_fields) = struct_item.fields() {
        for field in struct_fields.clone() {
            fields.push(quote! {
                #{ field.ident() }: input.parse()?,
            });
        }
    }

    Ok(quote! {
        impl #{ struct_item.generic_parameters() } Parse for #{ struct_item.ident() } #{ struct_item.generic_parameters() } {
            fn parse(input: &mut parsyng::parse::ParseBuffer) -> parsyng::error::Result<Self> {
                Ok(Self {
                    #fields
                })
            }
        }
    })
}
