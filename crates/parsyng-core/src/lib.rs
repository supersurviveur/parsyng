#![deny(clippy::all)]

// Put proc_macro in a private module to avoid being able to use `proc_macro::...` directly in this crate
// This way the `proc-macro2` feature will work out of the box.
#[cfg(not(feature = "proc-macro2"))]
mod sealed {
    pub extern crate proc_macro;
}
#[cfg(not(feature = "proc-macro2"))]
pub use sealed::proc_macro;

#[cfg(feature = "proc-macro2")]
pub use proc_macro2 as proc_macro;

pub mod ast;
pub mod combinator;
pub mod error;
pub mod parse;
pub mod proc_macro_ext;
pub mod quote;
pub mod span;

pub use parse::Parse;

// TODO: Try removing this and export it in `parsyng` to gain some compile time.
pub use parsyng_quote_macros::{quote, quote_spanned};

#[macro_export]
macro_rules! parse_quote {
    ($($t:tt)*) => {{
        let mut stream = $crate::parse::ParseBuffer::new($crate::quote! { $($t)* });
        stream.parse().unwrap()
    }};
}

#[macro_export]
macro_rules! format_ident {
    ($($args:tt)*) => {
        $crate::proc_macro::Ident::new(&format!($($args)*), $crate::proc_macro::Span::call_site())
    };
}

pub trait ToTokens {
    fn to_tokens(&self, tokens: &mut crate::proc_macro::TokenStream);

    fn to_token_stream(&self) -> crate::proc_macro::TokenStream {
        let mut token_stream = crate::proc_macro::TokenStream::new();
        self.to_tokens(&mut token_stream);
        token_stream
    }
}

#[doc(hidden)]
pub fn debug_stream(macro_name: &str, call_location: &str, input: &crate::proc_macro::TokenStream) {
    let output;
    #[cfg(feature = "debug-pretty")]
    {
        use std::{
            io::Write,
            path::PathBuf,
            process::{Command, Stdio},
        };

        fn catch_rustfmt_errors(input: &crate::proc_macro::TokenStream) -> Option<String> {
            // Wrap the input in a dummy function, otherwise statements like `let` can't be formatted
            let prefix = "fn __dummy() {\n";
            let suffix = "\n}";
            let input = format!("{}{}{}", prefix, input, suffix);

            let cargo = PathBuf::from(std::option_env!("CARGO")?);
            let mut rustfmt = cargo.parent()?.to_owned();
            rustfmt.push("rustfmt");

            let mut command = Command::new(rustfmt);
            let command = command.stdin(Stdio::piped()).stdout(Stdio::piped());
            let mut exec = command.spawn().ok()?;
            exec.stdin.take()?.write_all(input.as_bytes()).unwrap();
            let output = exec.wait_with_output().ok().and_then(|output| {
                if output.status.success() {
                    String::from_utf8(output.stdout).ok()
                } else {
                    None
                }
            })?;

            let output = output
                .trim()
                .strip_prefix(prefix)?
                .strip_suffix(suffix)?
                .trim();
            let output = output.replace("\n    ", "\n");

            Some(output)
        }

        output = catch_rustfmt_errors(input).unwrap_or(input.to_string());
    }
    #[cfg(not(feature = "debug-pretty"))]
    {
        output = input;
    }
    eprintln!(
        "[DEBUG] proc-macro `{}` called at {}\n{}",
        macro_name, call_location, output
    );
}
