use crate::ToTokens;

use crate::{
    Parse,
    ast::{attributes::parse_inner_attributes, item::Item},
};

#[derive(Clone, Debug)]
pub struct Crate {
    inner_attributes: Vec<crate::ast::attributes::Attribute>,
    items: Vec<Item>,
}

impl Parse for Crate {
    fn parse(input: &mut crate::parse::ParseBuffer) -> crate::error::Result<Self> {
        let inner_attributes = parse_inner_attributes(input);

        // Parse items, but tolerate stray empty brace groups between items (some std files have
        // adjacent macro/impl patterns that can leave empty groups in the token stream).
        let mut items = Vec::new();
        while !input.is_empty() {
            // Try parsing an item; if it fails and next token is an empty group, consume the group and continue.
            match input.try_parse::<crate::ast::item::Item>() {
                Ok(it) => items.push(it),
                Err(_) => {
                    // If next token is a Group, consume it and continue. This lets us tolerate
                    // macro-generated group tokens that appear between top-level items.
                    if let Some(_group) = input.peek_group() {
                        let _ = input.group();
                        continue;
                    }

                    // Sometimes the stream contains an *empty* pair of brace punctuations
                    // ('{' followed by '}') which some macro forms emit as separate puncts
                    // rather than a grouped TokenTree. Detect and skip that pattern here.
                    if let Some(p1) = input.peek_punct()
                        && p1.as_char() == '{'
                    {
                        // Clone and advance a fork to inspect the following token.
                        let mut fork = input.clone();
                        let _ = fork.next(); // consume the first punct in the fork
                        if let Some(p2) = fork.peek_punct()
                            && p2.as_char() == '}'
                        {
                            // consume both punctuations from the real input and continue
                            let _ = input.punct();
                            let _ = input.punct();
                            continue;
                        }
                    }

                    // re-run parse to get a proper diagnostic
                    return Err(crate::error::Diagnostics::new_error_spanned(
                        "Expected an item",
                        input.span(),
                    ));
                }
            }
        }

        Ok(Self {
            inner_attributes,
            items,
        })
    }
}

impl ToTokens for Crate {
    fn to_tokens(&self, tokens: &mut parsyng_quote::proc_macro::TokenStream) {
        self.inner_attributes.to_tokens(tokens);
        self.items.to_tokens(tokens);
    }
}
