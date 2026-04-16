use proc_macro::{Group, Ident, Punct, Spacing, Span, TokenStream, TokenTree};

#[proc_macro]
pub fn parsyng(input: TokenStream) -> TokenStream {
    parse_tokenstream(input)
}

fn parse_tokenstream(stream: TokenStream) -> TokenStream {
    let mut output: TokenStream = TokenStream::new();

    output.extend(
        "let mut tokens = parsyng_quote::proc_macro::TokenStream::new();".parse::<TokenStream>(),
    );

    let iter = stream.into_iter();

    for tt in iter {
        if match tt {
            TokenTree::Group(ref g) if g.delimiter() == proc_macro::Delimiter::Brace => {
                let mut inner = g.stream().into_iter();
                if let Some(TokenTree::Group(g)) = inner.next()
                    && g.delimiter() == proc_macro::Delimiter::Brace
                {
                    let mut args = TokenStream::new();

                    // Make `&{interpolation}, &mut tokens`
                    args.extend::<[TokenTree; _]>([
                        Punct::new('&', Spacing::Alone).into(),
                        Group::new(proc_macro::Delimiter::None, g.stream()).into(),
                        Punct::new(',', Spacing::Alone).into(),
                        Punct::new('&', Spacing::Alone).into(),
                        Ident::new("mut", Span::call_site()).into(),
                        Ident::new("tokens", Span::call_site()).into(),
                    ]);

                    // Make `::parsyng::ToTokens::to_tokens({args});`
                    // Or use parsyng_quote if not in the parsyng crate
                    output.extend::<[TokenTree; _]>([
                        Ident::new("parsyng_quote", Span::call_site()).into(),
                        Punct::new(':', Spacing::Joint).into(),
                        Punct::new(':', Spacing::Alone).into(),
                        Ident::new("ToTokens", Span::call_site()).into(),
                        Punct::new(':', Spacing::Joint).into(),
                        Punct::new(':', Spacing::Alone).into(),
                        Ident::new("to_tokens", Span::call_site()).into(),
                        Group::new(proc_macro::Delimiter::Parenthesis, args).into(),
                        Punct::new(';', Spacing::Alone).into(),
                    ]);

                    false
                } else {
                    true
                }
            }
            _ => true,
        } {
            token_to_construction_code(&mut output, tt);
        }
    }

    output.extend(core::iter::once(Ident::new("tokens", Span::call_site())));

    TokenTree::Group(Group::new(proc_macro::Delimiter::Brace, output)).into()
}

fn token_to_construction_code(output: &mut TokenStream, tt: TokenTree) {
    match tt {
        TokenTree::Group(group) => {
            let inner = parse_tokenstream(group.stream());
            output.extend(
                format!(
                    "parsyng_quote::__private::push_group(parsyng_quote::proc_macro::Delimiter::{:?}, {}, &mut tokens);",
                    group.delimiter(),
                    inner
                )
                .parse::<TokenStream>(),
            );
        }
        TokenTree::Ident(ident) => {
            let ident_string = ident.to_string();
            if let Some(raw_ident) = ident_string.strip_prefix("r#") {
                output.extend(
                    format!(
                        "parsyng_quote::__private::push_ident_raw(\"{}\", &mut tokens);",
                        raw_ident
                    )
                    .parse::<TokenStream>(),
                );
            } else {
                output.extend(
                    format!(
                        "parsyng_quote::__private::push_ident(\"{}\", &mut tokens);",
                        ident_string
                    )
                    .parse::<TokenStream>(),
                );
            }
        }
        TokenTree::Punct(punct) => match punct.spacing() {
            Spacing::Joint => output.extend(
                format!(
                    "parsyng_quote::__private::push_punct_joint('{}', &mut tokens);",
                    punct.as_char().escape_default(),
                )
                .parse::<TokenStream>(),
            ),
            Spacing::Alone => output.extend(
                format!(
                    "parsyng_quote::__private::push_punct_alone('{}', &mut tokens);",
                    punct.as_char().escape_default(),
                )
                .parse::<TokenStream>(),
            ),
        },
        TokenTree::Literal(literal) => {
            let literal = literal.to_string();
            let literal_escaped = literal.escape_default();
            output.extend(
                format!(
                    "parsyng_quote::__private::push_lit(\"{}\".parse::<parsyng_quote::proc_macro::TokenStream>().unwrap(), &mut tokens);",
                    literal_escaped,
                )
                .parse::<TokenStream>(),
            );
        }
    }
}
