use proc_macro::{Group, Ident, Literal, Punct, Spacing, Span, TokenStream, TokenTree};

const INTERPOLATION_CHAR: char = '#';

#[proc_macro]
pub fn parsyng(input: TokenStream) -> TokenStream {
    parse_tokenstream(input)
}

#[proc_macro]
pub fn parsyng_spanned(input: TokenStream) -> TokenStream {
    let mut span = TokenStream::new();
    let mut stream = input.into_iter();
    while let Some(tt) = stream.next()
        && match tt {
            TokenTree::Punct(ref punct) => punct.as_char() != '=',
            _ => true,
        }
    {
        span.extend(core::iter::once(tt));
    }

    let tt = stream.next();
    if match tt {
        Some(TokenTree::Punct(ref punct)) => punct.as_char() != '>',
        _ => false,
    } {
        let mut error = TokenStream::new();
        error.extend::<[TokenTree; _]>([
            Ident::new("compile_error", Span::call_site()).into(),
            Punct::new('!', Spacing::Alone).into(),
            Group::new(proc_macro::Delimiter::Brace, {
                let mut tk = TokenStream::new();
                tk.extend([Literal::string(&format!(
                    "expected '>', found '{}'",
                    tt.map_or("<eof>".to_string(), |tt| tt.to_string())
                ))]);
                tk
            })
            .into(),
        ]);
        return error;
    }

    parse_tokenstream(stream.collect())
}

fn parse_tokenstream(stream: TokenStream) -> TokenStream {
    let mut output: TokenStream = TokenStream::new();

    output.extend(
        "let mut tokens = parsyng_quote::proc_macro::TokenStream::new();".parse::<TokenStream>(),
    );

    let mut iter = stream.into_iter().peekable();

    while let Some(tt) = iter.next() {
        if let Some(interpolation) = match tt {
            TokenTree::Punct(ref punct)
                if punct.as_char() == INTERPOLATION_CHAR
                    && let Some(TokenTree::Ident(_)) = iter.peek() =>
            {
                iter.next()
            }
            TokenTree::Punct(ref punct)
                if punct.as_char() == INTERPOLATION_CHAR
                    && let Some(TokenTree::Group(g)) = iter.peek()
                    && g.delimiter() == proc_macro::Delimiter::Brace =>
            {
                let g = match iter.next().unwrap() {
                    TokenTree::Group(group) => group,
                    _ => unreachable!(),
                };
                Some(TokenTree::Group(Group::new(
                    proc_macro::Delimiter::None,
                    g.stream(),
                )))
            }
            _ => None,
        } {
            let mut args = TokenStream::new();

            // Make `&{interpolation}, &mut tokens`
            args.extend::<[TokenTree; _]>([
                Punct::new('&', Spacing::Alone).into(),
                interpolation,
                Punct::new(',', Spacing::Alone).into(),
                Punct::new('&', Spacing::Alone).into(),
                Ident::new("mut", Span::call_site()).into(),
                Ident::new("tokens", Span::call_site()).into(),
            ]);

            // Make `::parsyng::ToTokens::to_tokens({args});`
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
        } else {
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
