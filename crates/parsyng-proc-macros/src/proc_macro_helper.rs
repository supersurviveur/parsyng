use parsyng_core as parsyng;
use parsyng_core::ToTokens;

use parsyng_core::format_ident;
use parsyng_core::quote;
use parsyng_core::{
    Token,
    error::{self, Diagnostics},
    parse,
};
use proc_macro::Span;
use proc_macro::{Ident, TokenStream};

use crate::dbg_macros;

pub fn proc_macro(args: TokenStream, input: TokenStream) -> error::Result<TokenStream> {
    let mut stream = parse::ParseBuffer::new(input);
    let mut args = parse::ParseBuffer::new(args);

    stream.parse::<Token![pub]>()?;
    let signature = stream.parse::<parsyng_core::ast::signature::FnSignature>()?;
    let macro_ident = signature.ident();

    let params = signature.args();
    assert!(params.len() == 1);
    let param = params.iter().next().unwrap();
    let input_ident = param.ident();
    let in_type = param.ty();
    let out_type = signature.return_type().to_token_stream();

    // Create new function
    let new_macro_ident = format_ident!("__parsyng_{}", signature.ident());

    let dbg = if !args.is_empty() {
        let ident = args.parse::<Ident>()?;
        if ident.to_string() == "debug" {
            dbg_macros(
                macro_ident,
                format!(
                    "{}:{}:{}",
                    Span::call_site().file(),
                    Span::call_site().line(),
                    Span::call_site().column()
                ),
            )
        } else {
            return Err(Diagnostics::new_error_spanned(
                "Expected `debug` or no arguments.",
                ident.span(),
            ));
        }
    } else {
        TokenStream::new()
    };

    let new_function = quote! {
        #[proc_macro]
        pub fn #macro_ident(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
            let mut parse_buffer = parsyng::parse::ParseBuffer::new(input);
            let result = match <#in_type as parsyng::parse::Parse>::parse(&mut parse_buffer) {
                Ok(ok) => #new_macro_ident(ok),
                Err(err) => return <parsyng::error::Diagnostics as parsyng::ToTokens>::to_token_stream(&err)
            };
            let output = <#out_type as parsyng::ToTokens>::to_token_stream(&result);
            #dbg
            output
        }
    };

    Ok(quote! {
        #new_function

        fn #new_macro_ident(#input_ident: #in_type) -> #out_type #stream
    })
}
