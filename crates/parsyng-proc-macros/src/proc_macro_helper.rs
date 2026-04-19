use std::borrow::Cow;

use parsyng_quote::{format_ident, parsyng};
use proc_macro::{Delimiter, Span, TokenStream};

pub fn proc_macro(
    _args: TokenStream,
    input: TokenStream,
) -> Result<TokenStream, (Cow<'static, str>, Span)> {
    let mut stream = input.into_iter().peekable();

    // Parse the procedural_macro
    // pub fn {macro_ident}({input}: {in_type}) -> {out_type} {
    //     ...
    // }

    // pub
    match stream.next() {
        Some(proc_macro::TokenTree::Ident(ident)) if ident.to_string() == "pub" => {}
        other => {
            return Err((
                "functions tagged with `#[proc_macro]` must be `pub`".into(),
                other.map_or(Span::call_site(), |o| o.span()),
            ));
        }
    }
    // fn
    match stream.next() {
        Some(proc_macro::TokenTree::Ident(ident)) if ident.to_string() == "fn" => {}
        other => {
            return Err((
                "missing `fn` for function definition".into(),
                other.map_or(Span::call_site(), |o| o.span()),
            ));
        }
    }
    // {macro_ident}
    let macro_ident = parse_ident!(stream);
    // ({arguments})
    let mut arguments = match stream.next() {
        Some(proc_macro::TokenTree::Group(group))
            if group.delimiter() == Delimiter::Parenthesis =>
        {
            group.stream().into_iter().peekable()
        }
        other => {
            return Err((
                "expected block".into(),
                other.map_or(Span::call_site(), |o| o.span()),
            ));
        }
    };
    // {input}
    let input = parse_ident!(arguments);
    // :
    match arguments.next() {
        Some(proc_macro::TokenTree::Punct(punct)) if punct.as_char() == ':' => {}
        other => {
            expect_error!(":", other)
        }
    }
    // {in_type}
    let mut in_type = TokenStream::new();
    while let Some(tt) = arguments.peek()
        && !match tt {
            proc_macro::TokenTree::Punct(g) if g.as_char() == ',' => true,
            _ => false,
        }
    {
        in_type.extend(core::iter::once(arguments.next().unwrap()));
    }

    match arguments.next() {
        Some(other) => return Err(("Expected end of arguments".into(), other.span())),
        None => {}
    }

    // -
    match stream.next() {
        Some(proc_macro::TokenTree::Punct(punct)) if punct.as_char() == '-' => {}
        other => {
            expect_error!("-", other)
        }
    }
    match stream.next() {
        Some(proc_macro::TokenTree::Punct(punct)) if punct.as_char() == '>' => {}
        other => {
            expect_error!(">", other)
        }
    }
    // {out_type}
    let mut out_type = TokenStream::new();
    while let Some(tt) = stream.peek()
        && !match tt {
            proc_macro::TokenTree::Group(g) if g.delimiter() == Delimiter::Brace => true,
            _ => false,
        }
    {
        out_type.extend(core::iter::once(stream.next().unwrap()));
    }

    // Create new function
    let new_macro_ident = format_ident!("__parsyng_{}", macro_ident);

    let new_function = if out_type.to_string() == "TokenStream" {
        parsyng! {
            #[proc_macro]
            pub fn #{ macro_ident }(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
                let mut parse_buffer = parsyng::parse::ParseBuffer::new(input);
                match <#{ in_type } as parsyng::parse::Parse>::parse(&mut parse_buffer) {
                    Ok(ok) => #{ new_macro_ident }(ok),
                    Err(err) => {
                        let mut output = parsyng::proc_macro::TokenStream::new();
                        <#{ out_type } as parsyng::ToTokens>::to_tokens(&err, &mut output);
                        output
                    }
                }
            }
        }
    } else {
        parsyng! {
            #[proc_macro]
            pub fn #{ macro_ident }(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
                let mut parse_buffer = parsyng::parse::ParseBuffer::new(input);
                let result = match <#{ in_type } as parsyng::parse::Parse>::parse(&mut parse_buffer) {
                    Ok(ok) => #{ new_macro_ident }(ok),
                    Err(err) => Err(err),
                };
                let mut output = parsyng::proc_macro::TokenStream::new();
                <#{ out_type } as parsyng::ToTokens>::to_tokens(&result, &mut output);
                output
            }
        }
    };

    Ok(parsyng! {
        #{ new_function }

        fn #{ new_macro_ident }(#{ input }: #{ in_type }) -> #{ out_type } #{ stream.collect::<TokenStream>() }
    })
}
