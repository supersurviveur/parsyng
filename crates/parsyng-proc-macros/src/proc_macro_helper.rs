use parsyng_core::format_ident;
use parsyng_core::{
    Token,
    error::{self, Diagnostics},
    parse,
};
use parsyng_quote::quote;
use proc_macro::{Delimiter, Ident, TokenStream};

use crate::dbg_macros;

pub fn proc_macro(args: TokenStream, input: TokenStream) -> error::Result<TokenStream> {
    let mut stream = parse::ParseBuffer::new(input);
    let mut args = parse::ParseBuffer::new(args);

    stream.parse::<Token![pub]>()?;
    stream.parse::<Token![fn]>()?;
    let macro_ident = stream.parse::<proc_macro::Ident>()?;
    let mut arguments = parse::ParseBuffer::new(stream.parse::<proc_macro::Group>()?.stream());
    let input_ident = arguments.parse::<proc_macro::Ident>()?;
    arguments.parse::<Token![:]>()?;

    let mut in_type = TokenStream::new();
    while let Some(tt) = arguments.next() {
        if arguments.is_empty()
            && matches!(tt, proc_macro::TokenTree::Punct(ref g) if g.as_char() == ',')
        {
            break;
        }
        in_type.extend(Some(tt));
    }

    stream.parse::<Token![->]>().unwrap();

    let mut out_type = TokenStream::new();
    while let Some(tt) = stream.peek()
        && !matches!(tt, proc_macro::TokenTree::Group(g) if g.delimiter() == Delimiter::Brace)
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
        quote! {
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
        quote! {
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

    Ok(quote! {
        #{ new_function }

        fn #{ new_macro_ident }(#{ input_ident }: #{ in_type }) -> #{ out_type } #{ stream.collect::<TokenStream>() }
    })
}
