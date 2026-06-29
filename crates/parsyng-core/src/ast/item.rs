use core::ops::Deref;

use crate::{
    ToTokens,
    proc_macro::{Delimiter, TokenStream},
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
    TypeAlias(ItemTypeAlias),
    Use(ItemUse),
    ExternCrate(ItemExternCrate),
    ExternBlock(ItemExternBlock),
    Mod(ItemMod),
    Enum(ItemEnum),
    Function(ItemFunction),
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
    ident: Ident,
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
            Ok(Self::TypeAlias(VisItem {
                attributes,
                visibility,
                item: type_alias,
            }))
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
            Ok(Self::Function(VisItem {
                attributes,
                visibility,
                item: function_item,
            }))
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
impl ToTokens for Item {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Item::Struct(vis_item) => vis_item.to_tokens(tokens),
            Item::Const(vis_item) => vis_item.to_tokens(tokens),
            Item::TypeAlias(vis_item) => vis_item.to_tokens(tokens),
            Item::Use(vis_item) => vis_item.to_tokens(tokens),
            Item::ExternCrate(vis_item) => vis_item.to_tokens(tokens),
            Item::ExternBlock(vis_item) => vis_item.to_tokens(tokens),
            Item::Mod(vis_item) => vis_item.to_tokens(tokens),
            Item::Enum(vis_item) => vis_item.to_tokens(tokens),
            Item::Function(vis_item) => vis_item.to_tokens(tokens),
            Item::Trait(vis_item) => vis_item.to_tokens(tokens),
            Item::Static(vis_item) => vis_item.to_tokens(tokens),
            Item::MacroRules(vis_item) => vis_item.to_tokens(tokens),
            Item::Macro(vis_item) => vis_item.to_tokens(tokens),
            Item::MacroInvocation(macro_invocation) => macro_invocation.to_tokens(tokens),
            Item::Impl(implementation) => implementation.to_tokens(tokens),
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
#[allow(clippy::large_enum_variant)]
pub enum WhereClauseItem {
    Lifetime(LifetimeWhereClauseItem),
    Type(TypeBoundWhereClauseItem),
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
    start_token: Lt,
    generics: Punctuated<GenericParam, Comma, StopOnError>,
    last_token: Gt,
}

#[derive(Clone, Debug)]
pub enum GenericParam {
    Type(TypeParam),
    Lifetime(LifetimeParam),
    Const(ConstParam),
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
    Lifetime(Lifetime),
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

#[derive(Clone, Debug)]
pub struct Lifetime {
    quote: Quote,
    ident: Ident,
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
            WhereClauseItem::Lifetime(lifetime_where_clause_item) => {
                lifetime_where_clause_item.to_tokens(tokens)
            }
            WhereClauseItem::Type(type_bound_where_clause_item) => {
                type_bound_where_clause_item.to_tokens(tokens)
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
