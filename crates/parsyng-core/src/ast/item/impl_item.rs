use crate::ToTokens;

use crate::{
    ast::{
        attributes::parse_outer_attributes,
        item::{
            ItemFunction, associated::TypeAlias, constant::ConstantItem,
            macro_item::MacroInvocationItem,
        },
    },
    error::Diagnostics,
    parse::Parse,
};

#[derive(Clone, Debug)]
pub struct ImplItem {
    attributes: Vec<crate::ast::attributes::Attribute>,
    kind: ImplItemKind,
}

#[derive(Clone, Debug)]
pub enum ImplItemKind {
    Type(Box<TypeAlias>),
    Const(Box<ConstantItem>),
    Function(Box<ItemFunction>),
    Macro(MacroInvocationItem),
}

impl Parse for ImplItem {
    fn parse(input: &mut crate::parse::ParseBuffer) -> crate::error::Result<Self> {
        let attributes = parse_outer_attributes(input);
        let kind = if let Ok(item) = input.try_parse() {
            ImplItemKind::Type(item)
        } else if let Ok(item) = input.try_parse() {
            ImplItemKind::Const(item)
        } else if let Ok(item) = input.try_parse() {
            ImplItemKind::Function(item)
        } else if let Ok(item) = input.try_parse() {
            ImplItemKind::Macro(item)
        } else {
            return Err(Diagnostics::new_error_spanned(
                "Expected an impl item",
                input.span(),
            ));
        };
        Ok(Self { attributes, kind })
    }
}

impl ToTokens for ImplItem {
    fn to_tokens(&self, tokens: &mut crate::proc_macro::TokenStream) {
        self.attributes.to_tokens(tokens);
        self.kind.to_tokens(tokens);
    }
}

impl ToTokens for ImplItemKind {
    fn to_tokens(&self, tokens: &mut crate::proc_macro::TokenStream) {
        match self {
            Self::Type(item) => item.to_tokens(tokens),
            Self::Const(item) => item.to_tokens(tokens),
            Self::Function(item) => item.to_tokens(tokens),
            Self::Macro(item) => item.to_tokens(tokens),
        }
    }
}
