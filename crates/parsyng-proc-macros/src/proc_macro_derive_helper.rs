use parsyng_core as parsyng;
use parsyng_core::ToTokens;
use parsyng_core::ast::tokens::Comma;
use parsyng_core::proc_macro::Span;

use parsyng_core::format_ident;
use parsyng_core::quote;
use parsyng_core::{
    Token,
    error::{self, Diagnostics},
    parse,
};
use proc_macro::{Ident, TokenStream};

use crate::dbg_macros;

pub fn proc_macro_derive(args: TokenStream, input: TokenStream) -> error::Result<TokenStream> {
    let mut stream = parse::ParseBuffer::new(input);
    let mut args = parse::ParseBuffer::new(args);

    stream.parse::<Token![pub]>()?;
    let signature = stream.parse::<parsyng_core::ast::signature::FnSignature>()?;
    let macro_ident = signature.ident();

    let params = signature.args();
    assert!(params.len() == 1);
    let mut params = params.iter();

    let item_param = params.next().unwrap();
    let input_item_ident = item_param.ident();
    let input_item_mut = item_param.mutability();
    let item_type = item_param.ty();

    let out_type = signature.return_type().to_token_stream();

    // Create new function
    let new_macro_ident = format_ident!("__parsyng_{}", signature.ident());

    let derive_ident = args.parse::<Ident>()?;

    let dbg = if !args.is_empty() {
        args.parse::<Comma>()?;
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
        #[proc_macro_derive(#derive_ident)]
        pub fn #macro_ident(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
            let mut item_buffer = parsyng::parse::ParseBuffer::new(item);
            let result = match <#item_type as parsyng::parse::Parse>::parse(&mut item_buffer) {
                Ok(item) => #new_macro_ident(item),
                Err(err) => return <parsyng::error::Diagnostics as parsyng::ToTokens>::to_token_stream(&err),
            };
            let output = <#out_type as parsyng::ToTokens>::to_token_stream(&result);
            #dbg
            output
        }
    };

    Ok(quote! {
        #new_function

        fn #new_macro_ident(#input_item_mut #input_item_ident: #item_type) -> #out_type #stream
    })
}
