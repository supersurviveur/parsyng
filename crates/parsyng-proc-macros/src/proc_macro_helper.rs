use parsyng_quote::{format_ident, parsyng};
use proc_macro::{Delimiter, Ident, TokenStream};

use crate::{bootstrap, dbg_macros, error::Diagnostics};

pub fn proc_macro(args: TokenStream, input: TokenStream) -> bootstrap::error::Result<TokenStream> {
    let mut stream = bootstrap::parse::ParseBuffer::new(input);
    let mut args = bootstrap::parse::ParseBuffer::new(args);

    stream.parse::<Token![pub]>()?;
    stream.parse::<Token![fn]>()?;
    let macro_ident = stream.parse::<proc_macro::Ident>()?;
    let mut arguments =
        bootstrap::parse::ParseBuffer::new(stream.parse::<proc_macro::Group>()?.stream());
    let input_ident = arguments.parse::<proc_macro::Ident>()?;
    arguments.parse::<Token![:]>()?;

    let mut in_type = TokenStream::new();
    while let Some(tt) = arguments.next() {
        if arguments.is_empty()
            && match tt {
                proc_macro::TokenTree::Punct(ref g) if g.as_char() == ',' => true,
                _ => false,
            }
        {
            break;
        }
        in_type.extend(Some(tt));
    }

    stream.parse::<Token![->]>().unwrap();

    let mut out_type = TokenStream::new();
    while let Some(tt) = stream.peek()
        && !match tt {
            proc_macro::TokenTree::Group(g) if g.delimiter() == Delimiter::Brace => true,
            _ => false,
        }
    {
        out_type.extend(Some(stream.next().unwrap()));
    }

    // Create new function
    let new_macro_ident = format_ident!("__parsyng_{}", macro_ident);

    let dbg = if !args.is_empty() {
        let ident = args.parse::<Ident>()?;
        if ident.to_string() == "debug" {
            dbg_macros()
        } else {
            return Err(Diagnostics::new_error_spanned(
                "Expected `debug` or no arguments.",
                ident.span(),
            ));
        }
    } else {
        TokenStream::new()
    };

    let new_function = if out_type.to_string() == "TokenStream" {
        parsyng! {
            #[proc_macro]
            pub fn #{ macro_ident }(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
                let mut parse_buffer = parsyng::parse::ParseBuffer::new(input);
                match <#{ in_type } as parsyng::parse::Parse>::parse(&mut parse_buffer) {
                    Ok(ok) => {
                        let output = #{ new_macro_ident }(ok);
                        #dbg
                        output
                    },
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
                #dbg
                output
            }
        }
    };

    Ok(parsyng! {
        #{ new_function }

        fn #{ new_macro_ident }(#{ input_ident }: #{ in_type }) -> #{ out_type } #{ stream.collect::<TokenStream>() }
    })
}
