use core::ops::Deref;
use std::ops::DerefMut;

use crate::{
    ToTokens,
    ast::generics::{ImplGenerics, TypeGenerics},
    combinator::{PunctuatedIter, PunctuatedIterMut},
    proc_macro::{Delimiter, Span, TokenStream},
};

use crate::{
    ast::{
        attributes::{Attribute, parse_outer_attributes},
        item::{
            associated::TypeAlias,
            constant::ConstantItem,
            enum_item::EnumItem,
            extern_block::ExternBlockItem,
            extern_crate::ExternCrateItem,
            function::FunctionItem,
            implementation::Implementation,
            macro_item::{MacroInvocationItem, MacroItem, MacroRulesItem},
            module::ModItem,
            static_item::StaticItem,
            r#struct::Struct,
            trait_item::TraitItem,
            r#use::UseItem,
        },
        tokens::{Colon, Comma, Const, Eq, For, Gt, Lt, Plus, Question, Quote, Where},
        r#type::{Type, TypePath},
        visibility::Visibility,
    },
    combinator::{Punctuated, StopOnError},
    error::Diagnostics,
    parse::{Parse, ParseBuffer},
    proc_macro::{Group, Ident},
};

pub mod associated;
pub mod constant;
pub mod enum_item;
pub mod extern_block;
pub mod extern_crate;
pub mod function;
pub mod impl_item;
pub mod implementation;
pub mod macro_item;
pub mod module;
pub mod static_item;
pub mod r#struct;
pub mod trait_item;
pub mod r#use;

#[derive(Clone, Debug)]
pub enum Item {
    Struct(ItemStruct),
    Const(ItemConst),
    TypeAlias(Box<ItemTypeAlias>),
    Use(ItemUse),
    ExternCrate(ItemExternCrate),
    ExternBlock(ItemExternBlock),
    Mod(ItemMod),
    Enum(ItemEnum),
    Function(Box<ItemFunction>),
    Trait(ItemTrait),
    Static(ItemStatic),
    MacroRules(ItemMacroRules),
    Macro(ItemMacro),
    MacroInvocation(ItemMacroInvocation),
    Impl(ItemImpl),
}

#[derive(Clone, Debug)]
pub struct ConstParam {
    const_token: Const,
    pub ident: Ident,
    colon: Colon,
    ty: Type,
    default: Option<(Eq, Type)>,
}

#[derive(Clone, Debug)]
pub struct VisItem<T> {
    attributes: Vec<Attribute>,
    visibility: Visibility,
    item: T,
}

#[derive(Clone, Debug)]
pub enum DeriveInput {
    Struct(Box<ItemStruct>),
    Enum(Box<ItemEnum>),
}

impl DeriveInput {
    #[must_use]
    pub fn generics_parameters(&self) -> Option<&GenericParams> {
        match self {
            Self::Struct(vis_item) => vis_item.generic_parameters(),
            Self::Enum(_vis_item) => todo!(),
        }
    }
    pub fn generics_parameters_mut(&mut self) -> Option<&mut GenericParams> {
        match self {
            Self::Struct(vis_item) => vis_item.generic_parameters_mut(),
            Self::Enum(_vis_item) => todo!(),
        }
    }
    #[must_use]
    pub fn split_generics_for_impl(
        &self,
    ) -> (
        Option<ImplGenerics<'_>>,
        Option<TypeGenerics<'_>>,
        Option<&WhereClause>,
    ) {
        match self {
            Self::Struct(vis_item) => vis_item.split_generics_for_impl(),
            Self::Enum(_vis_item) => todo!(),
        }
    }
}

impl DeriveInput {
    #[must_use]
    pub fn ident(&self) -> &Ident {
        match self {
            Self::Struct(vis_item) => vis_item.ident(),
            Self::Enum(vis_item) => vis_item.ident(),
        }
    }
}

impl Parse for ConstParam {
    fn parse(input: &mut crate::parse::ParseBuffer) -> crate::error::Result<Self> {
        Ok(Self {
            const_token: input.parse()?,
            ident: input.parse()?,
            colon: input.parse()?,
            ty: input.parse()?,
            default: input.try_parse().ok(),
        })
    }
}

impl<T> Deref for VisItem<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.item
    }
}
impl<T> DerefMut for VisItem<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.item
    }
}

impl ToTokens for ConstParam {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.const_token.to_tokens(tokens);
        self.ident.to_tokens(tokens);
        self.colon.to_tokens(tokens);
        self.ty.to_tokens(tokens);
        self.default.to_tokens(tokens);
    }
}

impl<T: Parse> Parse for VisItem<T> {
    fn parse(input: &mut crate::parse::ParseBuffer) -> crate::error::Result<Self> {
        let attributes = parse_outer_attributes(input);
        Ok(Self {
            attributes,
            visibility: input.parse()?,
            item: input.parse()?,
        })
    }
}

impl<T: ToTokens> ToTokens for VisItem<T> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.attributes.to_tokens(tokens);
        self.visibility.to_tokens(tokens);
        self.item.to_tokens(tokens);
    }
}

impl Parse for Item {
    fn parse(input: &mut crate::parse::ParseBuffer) -> crate::error::Result<Self> {
        let attributes = parse_outer_attributes(input);
        let visibility = input.parse()?;
        if let Ok(r#struct) = input.try_parse() {
            Ok(Self::Struct(VisItem {
                attributes,
                visibility,
                item: r#struct,
            }))
        } else if let Ok(const_item) = input.try_parse() {
            Ok(Self::Const(VisItem {
                attributes,
                visibility,
                item: const_item,
            }))
        } else if let Ok(type_alias) = input.try_parse() {
            Ok(Self::TypeAlias(Box::new(VisItem {
                attributes,
                visibility,
                item: type_alias,
            })))
        } else if let Ok(r#use) = input.try_parse() {
            Ok(Self::Use(VisItem {
                attributes,
                visibility,
                item: r#use,
            }))
        } else if let Ok(extern_crate) = input.try_parse() {
            Ok(Self::ExternCrate(VisItem {
                attributes,
                visibility,
                item: extern_crate,
            }))
        } else if let Ok(extern_block) = input.try_parse() {
            Ok(Self::ExternBlock(VisItem {
                attributes,
                visibility,
                item: extern_block,
            }))
        } else if let Ok(module) = input.try_parse() {
            Ok(Self::Mod(VisItem {
                attributes,
                visibility,
                item: module,
            }))
        } else if let Ok(enum_item) = input.try_parse() {
            Ok(Self::Enum(VisItem {
                attributes,
                visibility,
                item: enum_item,
            }))
        } else if let Ok(function_item) = input.try_parse() {
            Ok(Self::Function(Box::new(VisItem {
                attributes,
                visibility,
                item: function_item,
            })))
        } else if let Ok(trait_item) = input.try_parse() {
            Ok(Self::Trait(VisItem {
                attributes,
                visibility,
                item: trait_item,
            }))
        } else if let Ok(static_item) = input.try_parse() {
            Ok(Self::Static(VisItem {
                attributes,
                visibility,
                item: static_item,
            }))
        } else if let Ok(implementation) = input.try_parse() {
            Ok(Self::Impl(VisItem {
                attributes,
                visibility,
                item: implementation,
            }))
        } else if let Ok(macro_rules) = input.try_parse() {
            Ok(Self::MacroRules(VisItem {
                attributes,
                visibility,
                item: macro_rules,
            }))
        } else if let Ok(macro_item) = input.try_parse() {
            Ok(Self::Macro(VisItem {
                attributes,
                visibility,
                item: macro_item,
            }))
        } else if let Ok(macro_invocation) = input.try_parse() {
            Ok(Self::MacroInvocation(VisItem {
                attributes,
                visibility,
                item: macro_invocation,
            }))
        } else {
            Err(Diagnostics::new_error_spanned(
                "Expected an item",
                input.span(),
            ))
        }
    }
}
impl Parse for DeriveInput {
    fn parse(input: &mut crate::parse::ParseBuffer) -> crate::error::Result<Self> {
        let attributes = parse_outer_attributes(input);
        let visibility = input.parse()?;
        if let Ok(r#struct) = input.try_parse() {
            Ok(Self::Struct(Box::new(VisItem {
                attributes,
                visibility,
                item: r#struct,
            })))
        } else if let Ok(enum_item) = input.try_parse() {
            Ok(Self::Enum(Box::new(VisItem {
                attributes,
                visibility,
                item: enum_item,
            })))
        } else {
            Err(Diagnostics::new_error_spanned(
                "Expected an derive input",
                input.span(),
            ))
        }
    }
}
impl ToTokens for Item {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Self::Struct(vis_item) => vis_item.to_tokens(tokens),
            Self::Const(vis_item) => vis_item.to_tokens(tokens),
            Self::TypeAlias(vis_item) => vis_item.to_tokens(tokens),
            Self::Use(vis_item) => vis_item.to_tokens(tokens),
            Self::ExternCrate(vis_item) => vis_item.to_tokens(tokens),
            Self::ExternBlock(vis_item) => vis_item.to_tokens(tokens),
            Self::Mod(vis_item) => vis_item.to_tokens(tokens),
            Self::Enum(vis_item) => vis_item.to_tokens(tokens),
            Self::Function(vis_item) => vis_item.to_tokens(tokens),
            Self::Trait(vis_item) => vis_item.to_tokens(tokens),
            Self::Static(vis_item) => vis_item.to_tokens(tokens),
            Self::MacroRules(vis_item) => vis_item.to_tokens(tokens),
            Self::Macro(vis_item) => vis_item.to_tokens(tokens),
            Self::MacroInvocation(macro_invocation) => macro_invocation.to_tokens(tokens),
            Self::Impl(implementation) => implementation.to_tokens(tokens),
        }
    }
}
impl ToTokens for DeriveInput {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Self::Struct(vis_item) => vis_item.to_tokens(tokens),
            Self::Enum(vis_item) => vis_item.to_tokens(tokens),
        }
    }
}

pub type ItemStruct = VisItem<Struct>;
pub type ItemConst = VisItem<ConstantItem>;
pub type ItemTypeAlias = VisItem<TypeAlias>;
pub type ItemUse = VisItem<UseItem>;
pub type ItemExternCrate = VisItem<ExternCrateItem>;
pub type ItemExternBlock = VisItem<ExternBlockItem>;
pub type ItemMod = VisItem<ModItem>;
pub type ItemEnum = VisItem<EnumItem>;
pub type ItemFunction = VisItem<FunctionItem>;
pub type ItemTrait = VisItem<TraitItem>;
pub type ItemStatic = VisItem<StaticItem>;
pub type ItemMacroRules = VisItem<MacroRulesItem>;
pub type ItemMacro = VisItem<MacroItem>;
pub type ItemMacroInvocation = VisItem<MacroInvocationItem>;
pub type ItemImpl = VisItem<Implementation>;

#[derive(Clone, Debug)]
pub struct WhereClause {
    where_keyword: Where,
    generics: Punctuated<WhereClauseItem, Comma, StopOnError>,
}

#[derive(Clone, Debug)]
pub enum WhereClauseItem {
    Lifetime(LifetimeWhereClauseItem),
    Type(Box<TypeBoundWhereClauseItem>),
}

#[derive(Clone, Debug)]
pub struct LifetimeWhereClauseItem {
    lifetime: Lifetime,
    colon: Colon,
    lifetime_bounds: Punctuated<Lifetime, Plus, StopOnError>,
}

#[derive(Clone, Debug)]
pub struct TypeBoundWhereClauseItem {
    for_lifetimes: Option<(For, GenericParams)>,
    ty: Type,
    colon: Colon,
    bounds: Option<TypeParamBounds>,
}

#[derive(Clone, Debug)]
pub struct GenericParams {
    pub start_token: Lt,
    pub generics: Punctuated<GenericParam, Comma, StopOnError>,
    pub last_token: Gt,
}

impl GenericParams {
    #[must_use]
    pub fn iter(&self) -> PunctuatedIter<'_, GenericParam, Comma> {
        self.generics.iter()
    }
    #[must_use]
    pub fn iter_mut(&mut self) -> PunctuatedIterMut<'_, GenericParam, Comma> {
        self.generics.iter_mut()
    }
}

impl<'a> IntoIterator for &'a GenericParams {
    type Item = &'a GenericParam;
    type IntoIter = PunctuatedIter<'a, GenericParam, Comma>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a> IntoIterator for &'a mut GenericParams {
    type Item = &'a mut GenericParam;
    type IntoIter = PunctuatedIterMut<'a, GenericParam, Comma>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

#[derive(Clone, Debug)]
pub enum GenericParam {
    Type(Box<TypeParam>),
    Lifetime(LifetimeParam),
    Const(Box<ConstParam>),
}

#[derive(Clone, Debug)]
pub struct TypeParam {
    pub ident: Ident,
    colon: Option<Colon>,
    pub bounds: TypeParamBounds,
    default: Option<(Eq, Type)>,
}

#[derive(Clone, Debug)]
pub struct TypeParamBounds {
    bounds: Punctuated<TypeParamBound, Plus, StopOnError>,
}

impl Default for TypeParamBounds {
    fn default() -> Self {
        Self::new()
    }
}

impl TypeParamBounds {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            bounds: Punctuated::new(),
        }
    }
    pub fn push(&mut self, bound: TypeParamBound) {
        let separator = Plus::new(bound.span());
        self.bounds.push((bound, separator));
    }
}

#[derive(Clone, Debug)]
pub enum TypeParamBound {
    Trait(Box<TraitBound>),
    Lifetime(Lifetime),
}

impl TypeParamBound {
    #[must_use]
    pub fn span(&self) -> Span {
        match self {
            Self::Trait(trait_bound) => trait_bound.span(),
            Self::Lifetime(lifetime) => lifetime.span(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct LifetimeParam {
    lifetime: Lifetime,
    bounds: Option<(Colon, LifetimeBounds)>,
}

#[derive(Clone, Debug)]
pub struct TraitBound {
    group: Option<Group>,
    question: Option<Question>,
    for_lifetimes: Option<(For, GenericParams)>,
    path: TypePath,
}

impl TraitBound {
    #[must_use]
    pub fn span(&self) -> Span {
        self.path.span()
    }
}

#[derive(Clone, Debug)]
pub struct Lifetime {
    quote: Quote,
    ident: Ident,
}

impl Lifetime {
    #[must_use]
    pub fn span(&self) -> Span {
        self.quote.span()
    }
}

#[derive(Clone, Debug)]
pub struct LifetimeBounds {
    bounds: Punctuated<Lifetime, Plus, StopOnError>,
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
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.where_keyword.to_tokens(tokens);
        self.generics.to_tokens(tokens);
    }
}

impl Parse for WhereClauseItem {
    fn parse(input: &mut crate::parse::ParseBuffer) -> crate::error::Result<Self> {
        if let Ok(lifetime) = input.try_parse() {
            Ok(Self::Lifetime(lifetime))
        } else if let Ok(ty) = input.try_parse() {
            Ok(Self::Type(ty))
        } else {
            Err(Diagnostics::new_error_spanned(
                "Expected a where clause item",
                input.span(),
            ))
        }
    }
}

impl ToTokens for WhereClauseItem {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Self::Lifetime(lifetime_where_clause_item) => {
                lifetime_where_clause_item.to_tokens(tokens);
            }
            Self::Type(type_bound_where_clause_item) => {
                type_bound_where_clause_item.to_tokens(tokens);
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
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.lifetime.to_tokens(tokens);
        self.colon.to_tokens(tokens);
        self.lifetime_bounds.to_tokens(tokens);
    }
}
impl Parse for TypeBoundWhereClauseItem {
    fn parse(input: &mut crate::parse::ParseBuffer) -> crate::error::Result<Self> {
        Ok(Self {
            for_lifetimes: input.try_parse().ok(),
            ty: input.parse()?,
            colon: input.parse()?,
            bounds: input.try_parse().ok(),
        })
    }
}

impl ToTokens for TypeBoundWhereClauseItem {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.for_lifetimes.to_tokens(tokens);
        self.ty.to_tokens(tokens);
        self.colon.to_tokens(tokens);
        self.bounds.to_tokens(tokens);
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
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.quote.to_tokens(tokens);
        self.ident.to_tokens(tokens);
    }
}

impl Parse for TypeParam {
    fn parse(input: &mut crate::parse::ParseBuffer) -> crate::error::Result<Self> {
        let ident = input.parse()?;

        let (colon, bounds) = if let Ok(colon) = input.peek_parse() {
            (Some(colon), input.parse()?)
        } else {
            (None, TypeParamBounds::new())
        };

        let default = if let Ok(eq) = input.peek_parse() {
            Some((eq, input.parse()?))
        } else {
            None
        };

        Ok(Self {
            ident,
            colon,
            bounds,
            default,
        })
    }
}

impl Parse for LifetimeBounds {
    fn parse(input: &mut crate::parse::ParseBuffer) -> crate::error::Result<Self> {
        let bounds: Punctuated<_, _, _> = input.parse()?;
        if bounds.is_empty() {
            Err(Diagnostics::new_error_spanned(
                "LifetimeBounds must not be empty !",
                input.span(),
            ))
        } else {
            Ok(Self { bounds })
        }
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
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.bounds.to_tokens(tokens);
    }
}
impl Parse for TypeParamBound {
    fn parse(input: &mut crate::parse::ParseBuffer) -> crate::error::Result<Self> {
        if let Ok(r#trait) = input.try_parse() {
            Ok(Self::Trait(r#trait))
        } else if let Ok(lifetime) = input.try_parse() {
            Ok(Self::Lifetime(lifetime))
        } else {
            Err(Diagnostics::new_error_spanned(
                "Expected a type parameter bound",
                input.span(),
            ))
        }
    }
}
impl ToTokens for TypeParamBound {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Self::Trait(trait_bound) => trait_bound.to_tokens(tokens),
            Self::Lifetime(lifetime) => lifetime.to_tokens(tokens),
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
    fn to_tokens(&self, tokens: &mut TokenStream) {
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
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.ident.to_tokens(tokens);
        self.colon.to_tokens(tokens);
        self.bounds.to_tokens(tokens);
        self.default.to_tokens(tokens);
    }
}
impl ToTokens for LifetimeParam {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.lifetime.to_tokens(tokens);
        self.bounds.to_tokens(tokens);
    }
}
impl ToTokens for LifetimeBounds {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.bounds.to_tokens(tokens);
    }
}

impl ToTokens for GenericParam {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Self::Type(ty) => ty.to_tokens(tokens),
            Self::Lifetime(lifetime_param) => lifetime_param.to_tokens(tokens),
            Self::Const(const_param) => const_param.to_tokens(tokens),
        }
    }
}

impl Parse for GenericParam {
    fn parse(input: &mut crate::parse::ParseBuffer) -> crate::error::Result<Self> {
        if let Ok(const_param) = input.try_parse() {
            Ok(Self::Const(const_param))
        } else if let Ok(lifetime_param) = input.try_parse() {
            Ok(Self::Lifetime(lifetime_param))
        } else if let Ok(ty) = input.try_parse() {
            Ok(Self::Type(ty))
        } else {
            Err(Diagnostics::new_error_spanned(
                "Expected a generic parameter",
                input.span(),
            ))
        }
    }
}
impl Parse for LifetimeParam {
    fn parse(input: &mut crate::parse::ParseBuffer) -> crate::error::Result<Self> {
        let lifetime = input.parse()?;

        let bounds = if let Ok(colon) = input.peek_parse() {
            Some((colon, input.parse()?))
        } else {
            None
        };
        Ok(Self { lifetime, bounds })
    }
}

impl ToTokens for GenericParams {
    fn to_tokens(&self, tokens: &mut TokenStream) {
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

#[cfg(test)]
mod tests {
    use crate as parsyng;
    use parsyng_quote_macros::quote;

    use super::*;
    use crate::ast::tests::check;

    #[test]
    fn test_type_path() {
        check::<TypePath>(quote! {
            FnOnce() -> T
        });
    }
}
