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
pub mod error;
pub mod parse;
pub mod proc_macro_ext;
pub mod span;

pub use parsyng_proc_macros::proc_macro;
pub use parsyng_quote::ToTokens;

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
