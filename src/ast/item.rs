use core::ops::Deref;

use parsyng_quote::{
    ToTokens,
    proc_macro::{Delimiter, TokenStream},
};

use crate::{
    ast::{
        item::r#struct::Struct,
        tokens::{Colon, Comma, Eq, For, Gt, Lt, Plus, Question, Quote, Type, Where},
        r#type::TypePath,
        visibility::Visibility,
    },
    combinator::{Punctuated, StopOnError},
    error::Diagnostics,
    parse::{Parse, ParseBuffer},
    proc_macro::{Group, Ident},
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

impl<T> Deref for VisItem<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.item
    }
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

impl Parse for Item {
    fn parse(input: &mut crate::parse::ParseBuffer) -> crate::error::Result<Self> {
        if let Ok(r#struct) = input.try_parse() {
            Ok(Self::Struct(r#struct))
        } else {
            Err(Diagnostics::new_error_spanned(
                "Expected an item",
                input.span(),
            ))
        }
    }
}
impl ToTokens for Item {
    fn to_tokens(&self, tokens: &mut parsyng_quote::proc_macro::TokenStream) {
        match self {
            Item::Struct(vis_item) => vis_item.to_tokens(tokens),
        }
    }
}

pub type ItemStruct = VisItem<Struct>;

#[derive(Clone, Debug)]
pub struct WhereClause {
    where_keyword: Where,
    generics: Punctuated<WhereClauseItem, Comma, StopOnError>,
}

#[derive(Clone, Debug)]
pub enum WhereClauseItem {
    Lifetime(LifetimeWhereClauseItem),
}

#[derive(Clone, Debug)]
pub struct LifetimeWhereClauseItem {
    lifetime: Lifetime,
    colon: Colon,
    lifetime_bounds: Punctuated<Lifetime, Plus, StopOnError>,
}

#[derive(Clone, Debug)]
pub struct GenericParams {
    start_token: Lt,
    generics: Punctuated<GenericParam, Comma, StopOnError>,
    last_token: Gt,
}

#[derive(Clone, Debug)]
pub enum GenericParam {
    Type(TypeParam),
}

#[derive(Clone, Debug)]
pub struct TypeParam {
    ident: Ident,
    bounds: Option<(Colon, TypeParamBounds)>,
    default: Option<(Eq, Type)>,
}

#[derive(Clone, Debug)]
pub struct TypeParamBounds {
    bounds: Punctuated<TypeParamBound, Plus, StopOnError>,
}

#[derive(Clone, Debug)]
pub enum TypeParamBound {
    Trait(Box<TraitBound>),
}
#[derive(Clone, Debug)]
pub struct TraitBound {
    group: Option<Group>,
    question: Option<Question>,
    for_lifetimes: Option<(For, GenericParams)>,
    path: TypePath,
}

#[derive(Clone, Debug)]
pub struct Lifetime {
    quote: Quote,
    ident: Ident,
}

impl Parse for WhereClause {
    fn parse(input: &mut crate::parse::ParseBuffer) -> crate::error::Result<Self> {
        Ok(Self {
            where_keyword: input.parse()?,
            generics: input.parse()?,
        })
    }
}

impl ToTokens for WhereClause {
    fn to_tokens(&self, tokens: &mut parsyng_quote::proc_macro::TokenStream) {
        self.where_keyword.to_tokens(tokens);
        self.generics.to_tokens(tokens);
    }
}

impl Parse for WhereClauseItem {
    fn parse(input: &mut crate::parse::ParseBuffer) -> crate::error::Result<Self> {
        Ok(Self::Lifetime(input.parse()?))
    }
}

impl ToTokens for WhereClauseItem {
    fn to_tokens(&self, tokens: &mut parsyng_quote::proc_macro::TokenStream) {
        match self {
            WhereClauseItem::Lifetime(lifetime_where_clause_item) => {
                lifetime_where_clause_item.to_tokens(tokens)
            }
        }
    }
}

impl Parse for LifetimeWhereClauseItem {
    fn parse(input: &mut crate::parse::ParseBuffer) -> crate::error::Result<Self> {
        Ok(Self {
            lifetime: input.parse()?,
            colon: input.parse()?,
            lifetime_bounds: input.parse()?,
        })
    }
}

impl ToTokens for LifetimeWhereClauseItem {
    fn to_tokens(&self, tokens: &mut parsyng_quote::proc_macro::TokenStream) {
        self.lifetime.to_tokens(tokens);
        self.colon.to_tokens(tokens);
        self.lifetime_bounds.to_tokens(tokens);
    }
}

impl Parse for Lifetime {
    fn parse(input: &mut crate::parse::ParseBuffer) -> crate::error::Result<Self> {
        Ok(Self {
            quote: input.parse()?,
            ident: input.parse()?,
        })
    }
}

impl ToTokens for Lifetime {
    fn to_tokens(&self, tokens: &mut parsyng_quote::proc_macro::TokenStream) {
        self.quote.to_tokens(tokens);
        self.ident.to_tokens(tokens);
    }
}

impl Parse for TypeParam {
    fn parse(input: &mut crate::parse::ParseBuffer) -> crate::error::Result<Self> {
        let ident = input.parse()?;

        let bounds = if let Ok(colon) = input.peek_parse() {
            Some((colon, input.parse()?))
        } else {
            None
        };

        let default = if let Ok(eq) = input.peek_parse() {
            Some((eq, input.parse()?))
        } else {
            None
        };

        Ok(Self {
            ident,
            bounds,
            default,
        })
    }
}

impl Parse for TypeParamBounds {
    fn parse(input: &mut crate::parse::ParseBuffer) -> crate::error::Result<Self> {
        let bounds: Punctuated<_, _, _> = input.parse()?;
        if bounds.is_empty() {
            Err(Diagnostics::new_error_spanned(
                "TypeParamBounds must not be empty !",
                input.span(),
            ))
        } else {
            Ok(Self { bounds })
        }
    }
}
impl ToTokens for TypeParamBounds {
    fn to_tokens(&self, tokens: &mut parsyng_quote::proc_macro::TokenStream) {
        self.bounds.to_tokens(tokens);
    }
}
impl Parse for TypeParamBound {
    fn parse(input: &mut crate::parse::ParseBuffer) -> crate::error::Result<Self> {
        Ok(Self::Trait(input.parse()?))
    }
}
impl ToTokens for TypeParamBound {
    fn to_tokens(&self, tokens: &mut parsyng_quote::proc_macro::TokenStream) {
        match self {
            Self::Trait(trait_bound) => trait_bound.to_tokens(tokens),
        }
    }
}
impl Parse for TraitBound {
    fn parse(input: &mut crate::parse::ParseBuffer) -> crate::error::Result<Self> {
        if let Some(group) = input.peek_group()
            && group.delimiter() == Delimiter::Parenthesis
        {
            let mut inner = ParseBuffer::new(group.stream());
            Ok(Self {
                group: input.group(),
                question: inner.peek_parse().ok(),
                for_lifetimes: inner.try_parse().ok(),
                path: inner.parse()?,
            })
        } else {
            Ok(Self {
                group: None,
                question: input.peek_parse().ok(),
                for_lifetimes: input.try_parse().ok(),
                path: input.parse()?,
            })
        }
    }
}

impl ToTokens for TraitBound {
    fn to_tokens(&self, tokens: &mut parsyng_quote::proc_macro::TokenStream) {
        if let Some(group) = &self.group {
            let mut inner_tokens = TokenStream::new();
            self.question.to_tokens(&mut inner_tokens);
            self.for_lifetimes.to_tokens(&mut inner_tokens);
            self.path.to_tokens(&mut inner_tokens);
            tokens.extend(Some(Group::new(group.delimiter(), inner_tokens)));
        } else {
            self.question.to_tokens(tokens);
            self.for_lifetimes.to_tokens(tokens);
            self.path.to_tokens(tokens);
        }
    }
}

impl ToTokens for TypeParam {
    fn to_tokens(&self, tokens: &mut parsyng_quote::proc_macro::TokenStream) {
        self.ident.to_tokens(tokens);
        self.bounds.to_tokens(tokens);
        self.default.to_tokens(tokens);
    }
}
impl ToTokens for GenericParam {
    fn to_tokens(&self, tokens: &mut parsyng_quote::proc_macro::TokenStream) {
        match self {
            Self::Type(ty) => ty.to_tokens(tokens),
        }
    }
}

impl Parse for GenericParam {
    fn parse(input: &mut crate::parse::ParseBuffer) -> crate::error::Result<Self> {
        Ok(Self::Type(input.parse()?))
    }
}
impl ToTokens for GenericParams {
    fn to_tokens(&self, tokens: &mut parsyng_quote::proc_macro::TokenStream) {
        self.start_token.to_tokens(tokens);
        self.generics.to_tokens(tokens);
        self.last_token.to_tokens(tokens);
    }
}
impl Parse for GenericParams {
    fn parse(input: &mut crate::parse::ParseBuffer) -> crate::error::Result<Self> {
        Ok(Self {
            start_token: input.parse()?,
            generics: input.parse()?,
            last_token: input.parse()?,
        })
    }
}
