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
pub mod span;

pub use parsyng_proc_macros::{Parse, ToTokens, proc_macro};
pub use parsyng_quote::ToTokens;

pub use parse::Parse;

pub use parsyng_quote;

#[macro_export]
macro_rules! parsyng {
    ($($t:tt)*) => {{
        use $crate::parsyng_quote;
        $crate::parsyng_quote::parsyng! {
            $($t)*
        }
    }};
}

#[doc(hidden)]
pub fn debug_stream(input: &crate::proc_macro::TokenStream) {
    #[cfg(feature = "debug-pretty")]
    if let Some(rustfmt) = toolchain_find::find_installed_component("rustfmt") {
        use std::{
            io::Write,
            path::PathBuf,
            process::{Command, Stdio},
        };

        fn catch_errors(
            rustfmt: PathBuf,
            input: &crate::proc_macro::TokenStream,
        ) -> Option<String> {
            // Wrap the input in a dummy function, otherwise statements like `let` can't be formatted
            let prefix = "fn __dummy() {\n";
            let suffix = "\n}";
            let input = format!("{}{}{}", prefix, input, suffix);

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

        println!(
            "{}",
            catch_errors(rustfmt, input).unwrap_or(input.to_string())
        );
    } else {
        println!("{}", input);
    };
    #[cfg(not(feature = "debug-pretty"))]
    println!("{}", input);
}
