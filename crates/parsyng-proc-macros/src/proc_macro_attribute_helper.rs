use parsyng_core as parsyng;
use parsyng_core::ToTokens;

use parsyng_core::format_ident;
use parsyng_core::quote;
use parsyng_core::{
    Token,
    error::{self, Diagnostics},
    parse,
};
use proc_macro::{Ident, TokenStream};

use crate::dbg_macros;

pub fn proc_macro_attribute(args: TokenStream, input: TokenStream) -> error::Result<TokenStream> {
    let mut stream = parse::ParseBuffer::new(input);
    let mut args = parse::ParseBuffer::new(args);

    stream.parse::<Token![pub]>()?;
    let signature = stream.parse::<parsyng_core::ast::signature::FnSignature>()?;
    let macro_ident = signature.ident();

    let params = signature.args();
    assert!(params.len() == 2);
    let mut params = params.iter();

    let attr_param = params.next().unwrap();
    let attr_ident = attr_param.ident();
    let attr_type = attr_param.ty();

    let item_param = params.next().unwrap();
    let input_item_ident = item_param.ident();
    let item_type = item_param.ty();

    let out_type = signature.return_type().to_token_stream();

    // Create new function
    let new_macro_ident = format_ident!("__parsyng_{}", signature.ident());

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

    let new_function = quote! {
        #[proc_macro_attribute]
        pub fn #macro_ident(attr: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
            let mut attr_buffer = parsyng::parse::ParseBuffer::new(attr);
            let mut item_buffer = parsyng::parse::ParseBuffer::new(item);
            let result = match (
                <#attr_type as parsyng::parse::Parse>::parse(&mut attr_buffer),
                <#item_type as parsyng::parse::Parse>::parse(&mut item_buffer)
            ) {
                (Ok(attr), Ok(item)) => #new_macro_ident(attr, item),
                (Err(mut err1), Err(err2)) => {
                    err1.join(err2);
                    return <parsyng::error::Diagnostics as parsyng::ToTokens>::to_token_stream(&err1);
                }
                (Err(err), _) => return <parsyng::error::Diagnostics as parsyng::ToTokens>::to_token_stream(&err),
                (_, Err(err)) => return <parsyng::error::Diagnostics as parsyng::ToTokens>::to_token_stream(&err),
            };
            let output = <#out_type as parsyng::ToTokens>::to_token_stream(&result);
            #dbg
            output
        }
    };

    Ok(quote! {
        #new_function

        fn #new_macro_ident(#attr_ident: #attr_type, #input_item_ident: #item_type) -> #out_type #stream
    })
}
