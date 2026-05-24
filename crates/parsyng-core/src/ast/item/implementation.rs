use parsyng_quote::ToTokens;

use crate::{
    ast::{
        delimiter::Braced,
        item::{GenericParams, WhereClause, associated::AssociatedAlias},
        tokens::{For, Impl, Not, Unsafe},
        r#type::{Type, TypePath},
    },
    parse::Parse,
};

#[derive(Clone, Debug)]
pub struct Implementation {
    unsafety: Option<Unsafe>,
    impl_token: Impl,
    generic_parameters: Option<GenericParams>,
    trait_impl: Option<(Option<Not>, TypePath, For)>,
    ty: Type,
    where_clause: Option<WhereClause>,
    associated_items: Braced<Vec<AssociatedAlias>>,
}

impl Parse for Implementation {
    fn parse(input: &mut crate::parse::ParseBuffer) -> crate::error::Result<Self> {
        Ok(Self {
            unsafety: input.parse()?,
            impl_token: input.parse()?,
            generic_parameters: input.try_parse().ok(),
            trait_impl: input.try_parse().ok(),
            ty: input.parse()?,
            where_clause: input.try_parse().ok(),
            associated_items: input.parse()?,
        })
    }
}

impl ToTokens for Implementation {
    fn to_tokens(&self, tokens: &mut parsyng_quote::proc_macro::TokenStream) {
        self.unsafety.to_tokens(tokens);
        self.impl_token.to_tokens(tokens);
        self.generic_parameters.to_tokens(tokens);
        self.trait_impl.to_tokens(tokens);
        self.ty.to_tokens(tokens);
        self.where_clause.to_tokens(tokens);
        self.associated_items.to_tokens(tokens);
    }
}
