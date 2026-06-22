use parsyng_quote::ToTokens;

use crate::{
    ast::{
        attributes::parse_outer_attributes,
        item::{
            associated::TypeAlias, constant::ConstantItem, function::FunctionItem,
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
    Type(TypeAlias),
    Const(ConstantItem),
    Function(FunctionItem),
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
    fn to_tokens(&self, tokens: &mut parsyng_quote::proc_macro::TokenStream) {
        self.attributes.to_tokens(tokens);
        self.kind.to_tokens(tokens);
    }
}

impl ToTokens for ImplItemKind {
    fn to_tokens(&self, tokens: &mut parsyng_quote::proc_macro::TokenStream) {
        match self {
            ImplItemKind::Type(item) => item.to_tokens(tokens),
            ImplItemKind::Const(item) => item.to_tokens(tokens),
            ImplItemKind::Function(item) => item.to_tokens(tokens),
            ImplItemKind::Macro(item) => item.to_tokens(tokens),
        }
    }
}
