use parsyng_quote::ToTokens;

use crate::{
    ast::{
        attributes::parse_outer_attributes,
        delimiter::Braced,
        item::{
            constant::ConstantItem,
            associated::TypeAlias,
        },
        signature::FnSignature,
        tokens::{Auto, Colon, Trait, Unsafe},
        item::TypeParamBounds,
    },
    error::Diagnostics,
    parse::Parse,
    proc_macro::{Delimiter, Ident, TokenStream},
};

#[derive(Clone, Debug)]
pub struct TraitItem {
    unsafety: Option<Unsafe>,
    auto_token: Option<Auto>,
    trait_token: Trait,
    ident: Ident,
    generics: Option<crate::ast::item::GenericParams>,
    bounds: Option<(Colon, TypeParamBounds)>,
    where_clause: Option<crate::ast::item::WhereClause>,
    items: Braced<Vec<TraitItemMember>>,
}

#[derive(Clone, Debug)]
pub struct TraitItemMember {
    attributes: Vec<crate::ast::attributes::Attribute>,
    kind: TraitItemKind,
}

#[derive(Clone, Debug)]
pub enum TraitItemKind {
    Type(TypeAlias),
    Const(ConstantItem),
    Function(TraitFunction),
}

#[derive(Clone, Debug)]
pub struct TraitFunction {
    signature: FnSignature,
    body: TraitFunctionBody,
}

#[derive(Clone, Debug)]
pub enum TraitFunctionBody {
    Block(Braced<TokenStream>),
    Semicolon(crate::ast::tokens::Semicolon),
}

impl Parse for TraitItem {
    fn parse(input: &mut crate::parse::ParseBuffer) -> crate::error::Result<Self> {
        let unsafety = input.try_parse().ok();
        let auto_token = input.try_parse().ok();
        Ok(Self {
            unsafety,
            auto_token,
            trait_token: input.parse()?,
            ident: input.parse()?,
            generics: input.try_parse().ok(),
            bounds: input.try_parse().ok(),
            where_clause: input.try_parse().ok(),
            items: input.parse()?,
        })
    }
}

impl Parse for TraitItemMember {
    fn parse(input: &mut crate::parse::ParseBuffer) -> crate::error::Result<Self> {
        let attributes = parse_outer_attributes(input);
        let kind = if let Ok(item) = input.try_parse() {
            TraitItemKind::Type(item)
        } else if let Ok(item) = input.try_parse() {
            TraitItemKind::Const(item)
        } else if let Ok(item) = input.try_parse() {
            TraitItemKind::Function(item)
        } else {
            return Err(Diagnostics::new_error_spanned(
                "Expected a trait item",
                input.span(),
            ));
        };
        Ok(Self { attributes, kind })
    }
}

impl Parse for TraitFunction {
    fn parse(input: &mut crate::parse::ParseBuffer) -> crate::error::Result<Self> {
        let signature: FnSignature = input.parse()?;
        let body = if let Some(group) = input.peek_group()
            && group.delimiter() == Delimiter::Brace
        {
            TraitFunctionBody::Block(input.parse()?)
        } else {
            TraitFunctionBody::Semicolon(input.parse()?)
        };
        Ok(Self { signature, body })
    }
}

impl ToTokens for TraitItem {
    fn to_tokens(&self, tokens: &mut parsyng_quote::proc_macro::TokenStream) {
        self.unsafety.to_tokens(tokens);
        self.auto_token.to_tokens(tokens);
        self.trait_token.to_tokens(tokens);
        self.ident.to_tokens(tokens);
        self.generics.to_tokens(tokens);
        self.bounds.to_tokens(tokens);
        self.where_clause.to_tokens(tokens);
        self.items.to_tokens(tokens);
    }
}

impl ToTokens for TraitItemMember {
    fn to_tokens(&self, tokens: &mut parsyng_quote::proc_macro::TokenStream) {
        self.attributes.to_tokens(tokens);
        self.kind.to_tokens(tokens);
    }
}

impl ToTokens for TraitItemKind {
    fn to_tokens(&self, tokens: &mut parsyng_quote::proc_macro::TokenStream) {
        match self {
            TraitItemKind::Type(item) => item.to_tokens(tokens),
            TraitItemKind::Const(item) => item.to_tokens(tokens),
            TraitItemKind::Function(item) => item.to_tokens(tokens),
        }
    }
}

impl ToTokens for TraitFunction {
    fn to_tokens(&self, tokens: &mut parsyng_quote::proc_macro::TokenStream) {
        self.signature.to_tokens(tokens);
        self.body.to_tokens(tokens);
    }
}

impl ToTokens for TraitFunctionBody {
    fn to_tokens(&self, tokens: &mut parsyng_quote::proc_macro::TokenStream) {
        match self {
            TraitFunctionBody::Block(block) => block.to_tokens(tokens),
            TraitFunctionBody::Semicolon(semicolon) => semicolon.to_tokens(tokens),
        }
    }
}
