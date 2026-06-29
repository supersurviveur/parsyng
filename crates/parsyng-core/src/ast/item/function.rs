use crate::ToTokens;

use crate::{
    ast::{delimiter::Braced, signature::FnSignature, tokens::Semicolon},
    parse::Parse,
    proc_macro::{Delimiter, TokenStream},
};

#[derive(Clone, Debug)]
pub struct FunctionItem {
    signature: FnSignature,
    body: FunctionBody,
}

#[derive(Clone, Debug)]
pub enum FunctionBody {
    Block(Braced<TokenStream>),
    Semicolon(Semicolon),
}

impl FunctionItem {
    pub fn signature(&self) -> &FnSignature {
        &self.signature
    }
}

impl Parse for FunctionItem {
    fn parse(input: &mut crate::parse::ParseBuffer) -> crate::error::Result<Self> {
        let signature = input.parse()?;
        let body = if let Some(group) = input.peek_group()
            && group.delimiter() == Delimiter::Brace
        {
            FunctionBody::Block(input.parse()?)
        } else {
            FunctionBody::Semicolon(input.parse()?)
        };
        Ok(Self { signature, body })
    }
}

impl ToTokens for FunctionItem {
    fn to_tokens(&self, tokens: &mut crate::proc_macro::TokenStream) {
        self.signature.to_tokens(tokens);
        self.body.to_tokens(tokens);
    }
}

impl ToTokens for FunctionBody {
    fn to_tokens(&self, tokens: &mut crate::proc_macro::TokenStream) {
        match self {
            FunctionBody::Block(block) => block.to_tokens(tokens),
            FunctionBody::Semicolon(semicolon) => semicolon.to_tokens(tokens),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate as parsyng;
    use parsyng_quote_macros::quote;

    use super::*;
    use crate::ast::tests::check;

    #[test]
    fn test_impl_arguments() {
        check::<FunctionItem>(quote! {
            fn with<T>(&self, f: impl FnOnce() -> T) -> T {
                CURRENT.with(|local_data| {
                    let _guard = local_data.enter(self.context.clone());
                    f()
                })
            }
        });
    }
}
