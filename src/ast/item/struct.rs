use parsyng_quote::ToTokens;

use crate::{
    ast::{
        delimiter::Braced,
        tokens::{Colon, Comma, StructKeyword},
        r#type::Type,
        visibility::Visibility,
    },
    combinator::Punctuated,
    parse::Parse,
    proc_macro::Ident,
};

#[derive(Clone, Debug)]
pub struct Struct {
    struct_token: StructKeyword,
    struct_ident: Ident,
    fields: Braced<Punctuated<StructField, Comma>>,
}

impl Struct {
    pub fn ident(&self) -> &Ident {
        &self.struct_ident
    }
    pub fn fields(&self) -> &Punctuated<StructField, Comma> {
        &self.fields
    }
}

impl Parse for Struct {
    fn parse(input: &mut crate::parse::ParseBuffer) -> crate::error::Result<Self> {
        Ok(Self {
            struct_token: input.parse()?,
            struct_ident: input.parse()?,
            fields: input.parse()?,
        })
    }
}

impl ToTokens for Struct {
    fn to_tokens(&self, tokens: &mut parsyng_quote::proc_macro::TokenStream) {
        self.struct_token.to_tokens(tokens);
        self.struct_ident.to_tokens(tokens);
        self.fields.to_tokens(tokens);
    }
}

#[derive(Clone, Debug)]
pub struct StructField {
    visibility: Visibility,
    field_ident: Ident,
    colon_token: Colon,
    ty: Type,
}

impl StructField {
    pub fn ident(&self) -> &Ident {
        &self.field_ident
    }
}

impl Parse for StructField {
    fn parse(input: &mut crate::parse::ParseBuffer) -> crate::error::Result<Self> {
        Ok(Self {
            visibility: input.parse()?,
            field_ident: input.parse()?,
            colon_token: input.parse()?,
            ty: input.parse()?,
        })
    }
}
impl ToTokens for StructField {
    fn to_tokens(&self, tokens: &mut parsyng_quote::proc_macro::TokenStream) {
        self.visibility.to_tokens(tokens);
        self.field_ident.to_tokens(tokens);
        self.colon_token.to_tokens(tokens);
        self.ty.to_tokens(tokens);
    }
}
