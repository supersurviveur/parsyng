use parsyng_quote::ToTokens;

use crate::{
    ast::{item::r#struct::Struct, visibility::Visibility},
    parse::Parse,
};

pub mod r#struct;

#[derive(Clone, Debug)]
pub enum Item {
    Struct(ItemStruct),
}

#[derive(Clone, Debug)]
pub struct VisItem<T> {
    visibility: Visibility,
    item: T,
}

impl<T: Parse> Parse for VisItem<T> {
    fn parse(input: &mut crate::parse::ParseBuffer) -> crate::error::Result<Self> {
        Ok(Self {
            visibility: input.parse()?,
            item: input.parse()?,
        })
    }
}

impl<T: ToTokens> ToTokens for VisItem<T> {
    fn to_tokens(&self, tokens: &mut parsyng_quote::proc_macro::TokenStream) {
        self.visibility.to_tokens(tokens);
        self.item.to_tokens(tokens);
    }
}

pub type ItemStruct = VisItem<Struct>;
