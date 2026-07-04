use parsyng_core as parsyng;
use parsyng_core::ast::item::r#struct::StructFields;

use parsyng_core::quote;
use parsyng_core::{ast::item::ItemStruct, error, parse};
use proc_macro::TokenStream;

pub fn derive_parse(input: TokenStream) -> error::Result<TokenStream> {
    let mut stream = parse::ParseBuffer::new(input);

    let struct_item = stream.parse::<ItemStruct>()?;

    let mut fields = vec![];

    match &struct_item.fields {
        StructFields::Named(struct_fields) => {
            for field in struct_fields.iter() {
                fields.push(quote! {
                    #{ field.ident() }: input.parse()?,
                });
            }
        }
        StructFields::Unnamed(_) => todo!(),
        StructFields::Unit => todo!(),
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
