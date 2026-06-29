use crate::ToTokens;

use crate::{
    ast::{
        delimiter::Braced,
        tokens::{As, Comma, PathSep, Semicolon, Star, Use},
    },
    combinator::Punctuated,
    parse::Parse,
    proc_macro::Ident,
};

#[derive(Clone, Debug)]
pub struct UseItem {
    use_token: Use,
    tree: UseTree,
    semi: Semicolon,
}

#[derive(Clone, Debug)]
pub enum UseTree {
    Path(UsePath),
    Name(Ident),
    Rename(UseRename),
    Glob(Star),
    Group(Box<UseGroup>),
}

#[derive(Clone, Debug)]
pub struct UsePath {
    ident: Ident,
    colon: PathSep,
    tree: Box<UseTree>,
}

#[derive(Clone, Debug)]
pub struct UseRename {
    ident: Ident,
    as_token: As,
    rename: Ident,
}

#[derive(Clone, Debug)]
pub struct UseGroup {
    group: Braced<Punctuated<UseTree, Comma>>,
}

impl Parse for UseItem {
    fn parse(input: &mut crate::parse::ParseBuffer) -> crate::error::Result<Self> {
        Ok(Self {
            use_token: input.parse()?,
            tree: input.parse()?,
            semi: input.parse()?,
        })
    }
}

impl Parse for UseTree {
    fn parse(input: &mut crate::parse::ParseBuffer) -> crate::error::Result<Self> {
        if let Ok(group) = input.try_parse() {
            return Ok(Self::Group(Box::new(UseGroup { group })));
        }
        if let Ok(star) = input.try_parse() {
            return Ok(Self::Glob(star));
        }

        let ident: Ident = input.parse()?;
        if let Ok(as_token) = input.try_parse() {
            return Ok(Self::Rename(UseRename {
                ident,
                as_token,
                rename: input.parse()?,
            }));
        }
        if let Ok(colon) = input.try_parse::<PathSep>() {
            let tree = if let Ok(group) = input.try_parse() {
                UseTree::Group(Box::new(UseGroup { group }))
            } else if let Ok(star) = input.try_parse() {
                UseTree::Glob(star)
            } else {
                input.parse()?
            };
            return Ok(Self::Path(UsePath {
                ident,
                colon,
                tree: Box::new(tree),
            }));
        }

        Ok(Self::Name(ident))
    }
}

impl ToTokens for UseItem {
    fn to_tokens(&self, tokens: &mut crate::proc_macro::TokenStream) {
        self.use_token.to_tokens(tokens);
        self.tree.to_tokens(tokens);
        self.semi.to_tokens(tokens);
    }
}

impl ToTokens for UseTree {
    fn to_tokens(&self, tokens: &mut crate::proc_macro::TokenStream) {
        match self {
            UseTree::Path(path) => path.to_tokens(tokens),
            UseTree::Name(name) => name.to_tokens(tokens),
            UseTree::Rename(rename) => rename.to_tokens(tokens),
            UseTree::Glob(glob) => glob.to_tokens(tokens),
            UseTree::Group(group) => group.to_tokens(tokens),
        }
    }
}

impl ToTokens for UsePath {
    fn to_tokens(&self, tokens: &mut crate::proc_macro::TokenStream) {
        self.ident.to_tokens(tokens);
        self.colon.to_tokens(tokens);
        self.tree.to_tokens(tokens);
    }
}

impl ToTokens for UseRename {
    fn to_tokens(&self, tokens: &mut crate::proc_macro::TokenStream) {
        self.ident.to_tokens(tokens);
        self.as_token.to_tokens(tokens);
        self.rename.to_tokens(tokens);
    }
}

impl ToTokens for UseGroup {
    fn to_tokens(&self, tokens: &mut crate::proc_macro::TokenStream) {
        self.group.to_tokens(tokens);
    }
}
