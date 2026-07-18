use crate::ToTokens;

use crate::ast::attributes::Attribute;
use crate::ast::generics::{ImplGenerics, TypeGenerics};
use crate::{
    ast::{
        attributes::parse_outer_attributes,
        delimiter::{Braced, Parenthesized},
        item::{GenericParams, WhereClause},
        tokens::{Colon, Comma, Semicolon, StructKeyword},
        r#type::Type,
        visibility::Visibility,
    },
    combinator::Punctuated,
    parse::Parse,
    proc_macro::{Delimiter, Ident, Span},
};

#[derive(Clone, Debug)]
pub struct Struct {
    #[allow(clippy::struct_field_names)]
    struct_token: StructKeyword,
    ident: Ident,
    pub generic_parameters: Option<GenericParams>,
    where_clause: Option<WhereClause>,
    pub fields: StructFields,
    semicolon: Option<Semicolon>,
}

impl Struct {
    #[must_use]
    pub const fn ident(&self) -> &Ident {
        &self.ident
    }
    #[must_use]
    pub const fn generic_parameters(&self) -> Option<&GenericParams> {
        self.generic_parameters.as_ref()
    }
    pub const fn generic_parameters_mut(&mut self) -> Option<&mut GenericParams> {
        self.generic_parameters.as_mut()
    }
    pub fn split_generics_for_impl(
        &self,
    ) -> (
        Option<ImplGenerics<'_>>,
        Option<TypeGenerics<'_>>,
        Option<&WhereClause>,
    ) {
        (
            self.generic_parameters().map(Into::into),
            self.generic_parameters().map(Into::into),
            self.where_clause.as_ref(),
        )
    }
}


impl Parse for Struct {
    fn parse(input: &mut crate::parse::ParseBuffer) -> crate::error::Result<Self> {
        let struct_token = input.parse()?;
        let struct_ident = input.parse()?;
        let generic_parameters = input.try_parse().ok();
        let where_clause = input.try_parse().ok();
        let (fields, semicolon) = if let Some(group) = input.peek_group()
            && group.delimiter() == Delimiter::Brace
        {
            (StructFields::Named(input.parse()?), None)
        } else if let Some(group) = input.peek_group()
            && group.delimiter() == Delimiter::Parenthesis
        {
            (StructFields::Unnamed(input.parse()?), Some(input.parse()?))
        } else {
            (StructFields::Unit, Some(input.parse()?))
        };

        Ok(Self {
            struct_token,
            ident: struct_ident,
            generic_parameters,
            where_clause,
            fields,
            semicolon,
        })
    }
}

impl ToTokens for Struct {
    fn to_tokens(&self, tokens: &mut crate::proc_macro::TokenStream) {
        self.struct_token.to_tokens(tokens);
        self.ident.to_tokens(tokens);
        self.generic_parameters.to_tokens(tokens);
        self.where_clause.to_tokens(tokens);
        self.fields.to_tokens(tokens);
        self.semicolon.to_tokens(tokens);
    }
}

#[derive(Clone, Debug)]
pub struct StructField {
    attributes: Vec<Attribute>,
    visibility: Visibility,
    pub ident: Ident,
    colon_token: Colon,
    ty: Type,
}

#[derive(Clone, Debug)]
pub enum StructFields {
    Named(Box<Braced<Punctuated<StructField, Comma>>>),
    Unnamed(Box<Parenthesized<Punctuated<TupleField, Comma>>>),
    Unit,
}

#[derive(Clone, Debug)]
pub struct TupleField {
    attributes: Vec<crate::ast::attributes::Attribute>,
    ty: Type,
}

impl StructField {
    #[must_use]
    pub fn span(&self) -> Span {
        self.ident.span()
    }
    #[must_use]
    pub const fn ident(&self) -> &Ident {
        &self.ident
    }
}

impl TupleField {
    #[must_use]
    pub fn span(&self) -> Span {
        self.ty.span()
    }
}

impl Parse for StructField {
    fn parse(input: &mut crate::parse::ParseBuffer) -> crate::error::Result<Self> {
        let attributes = parse_outer_attributes(input);
        Ok(Self {
            attributes,
            visibility: input.parse()?,
            ident: input.parse()?,
            colon_token: input.parse()?,
            ty: input.parse()?,
        })
    }
}

impl Parse for TupleField {
    fn parse(input: &mut crate::parse::ParseBuffer) -> crate::error::Result<Self> {
        Ok(Self {
            attributes: parse_outer_attributes(input),
            ty: input.parse()?,
        })
    }
}

impl ToTokens for StructField {
    fn to_tokens(&self, tokens: &mut crate::proc_macro::TokenStream) {
        self.attributes.to_tokens(tokens);
        self.visibility.to_tokens(tokens);
        self.ident.to_tokens(tokens);
        self.colon_token.to_tokens(tokens);
        self.ty.to_tokens(tokens);
    }
}

impl ToTokens for StructFields {
    fn to_tokens(&self, tokens: &mut crate::proc_macro::TokenStream) {
        match self {
            Self::Named(fields) => fields.to_tokens(tokens),
            Self::Unnamed(fields) => fields.to_tokens(tokens),
            Self::Unit => {}
        }
    }
}

impl ToTokens for TupleField {
    fn to_tokens(&self, tokens: &mut crate::proc_macro::TokenStream) {
        self.attributes.to_tokens(tokens);
        self.ty.to_tokens(tokens);
    }
}

#[cfg(test)]
mod tests {
    use crate as parsyng;
    use parsyng_quote_macros::quote;

    use super::*;
    use crate::ast::tests::check;

    #[test]
    fn test_doc_attributes() {
        check::<Struct>(quote! {
            struct Receiver<T> {
                /// State shared with all receivers and senders.
                shared: Arc<Shared<T>>,

                /// Next position to read from
                next: u64,
            }
        });
    }
}
