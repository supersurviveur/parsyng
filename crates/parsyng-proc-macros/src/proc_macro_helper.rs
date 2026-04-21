use parsyng_quote::{format_ident, parsyng};
use proc_macro::{Delimiter, TokenStream};

use crate::bootstrap;

pub fn proc_macro(_args: TokenStream, input: TokenStream) -> bootstrap::error::Result<TokenStream> {
    let mut stream = bootstrap::parse::ParseBuffer::new(input);

    stream.parse::<Token![pub]>()?;
    stream.parse::<Token![fn]>()?;
    let macro_ident = stream.parse::<proc_macro::Ident>()?;
    let mut arguments =
        bootstrap::parse::ParseBuffer::new(stream.parse::<proc_macro::Group>()?.stream());
    let input_ident = arguments.parse::<proc_macro::Ident>()?;
    arguments.parse::<Token![:]>()?;

    let mut in_type = TokenStream::new();
    while let Some(tt) = arguments.next()
    // && !match tt {
    //     proc_macro::TokenTree::Punct(ref g) if g.as_char() == ',' => true,
    //     _ => false,
    // }
    {
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

    // TODO, find a way to parse only the first arg, maybe when dependency tree will be inversed
    // if !arguments.is_empty() {
    //     return Err(Diagnostics::new_error_spanned(
    //         "Expected end of arguments",
    //         arguments.span(),
    //     ));
    // }

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

        fn #{ new_macro_ident }(#{ input_ident }: #{ in_type }) -> #{ out_type } #{ stream.collect::<TokenStream>() }
    })
}
