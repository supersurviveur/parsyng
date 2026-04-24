use parsyng_quote::ToTokens;

use crate::{
    ast::{
        item::{GenericParams, TypeParamBounds, WhereClause, constant::ConstantItem},
        tokens::{self, Colon, Eq, Semicolon},
        r#type::Type,
        visibility::Visibility,
    },
    error::Diagnostics,
    parse::Parse,
    proc_macro::Ident,
};

#[derive(Clone, Debug)]
pub struct TypeAlias {
    type_token: tokens::Type,
    ident: Ident,
    generics_parameters: Option<GenericParams>,
    bounds: Option<(Colon, TypeParamBounds)>,
    where_clause: Option<WhereClause>,
    default: Option<(Eq, Type, Option<WhereClause>)>,
    semi: Semicolon,
}

#[derive(Clone, Debug)]
pub enum AssociatedAlias {
    TypeAlias(Visibility, TypeAlias),
    Const(Visibility, ConstantItem),
}

impl Parse for TypeAlias {
    fn parse(input: &mut crate::parse::ParseBuffer) -> crate::error::Result<Self> {
        let type_token = input.parse()?;
        let ident = input.parse()?;
        let generics_parameters = input.try_parse().ok();
        let bounds = input.try_parse().ok();
        let where_clause = input.try_parse().ok();

        let default = if let Ok(eq) = input.peek_parse() {
            Some((eq, input.parse()?, input.try_parse().ok()))
        } else {
            None
        };
        Ok(Self {
            type_token,
            ident,
            generics_parameters,
            bounds,
            where_clause,
            default,
            semi: input.parse()?,
        })
    }
}

impl ToTokens for TypeAlias {
    fn to_tokens(&self, tokens: &mut parsyng_quote::proc_macro::TokenStream) {
        self.type_token.to_tokens(tokens);
        self.ident.to_tokens(tokens);
        self.generics_parameters.to_tokens(tokens);
        self.bounds.to_tokens(tokens);
        self.where_clause.to_tokens(tokens);
        self.default.to_tokens(tokens);
        self.semi.to_tokens(tokens);
    }
}

impl Parse for AssociatedAlias {
    fn parse(input: &mut crate::parse::ParseBuffer) -> crate::error::Result<Self> {
        if let Ok((visibility, ty_alias)) = input.try_parse() {
            Ok(Self::TypeAlias(visibility, ty_alias))
        } else if let Ok((visibility, constant)) = input.try_parse() {
            Ok(Self::Const(visibility, constant))
        } else {
            Err(Diagnostics::new_error_spanned(
                "Expected an item",
                input.span(),
            ))
        }
    }
}

impl ToTokens for AssociatedAlias {
    fn to_tokens(&self, tokens: &mut parsyng_quote::proc_macro::TokenStream) {
        match self {
            AssociatedAlias::TypeAlias(visibility, type_alias) => {
                visibility.to_tokens(tokens);
                type_alias.to_tokens(tokens);
            }
            AssociatedAlias::Const(visibility, constant_item) => {
                visibility.to_tokens(tokens);
                constant_item.to_tokens(tokens);
            }
        }
    }
}
