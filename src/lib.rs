#![deny(clippy::all)]

#[cfg(not(feature = "proc-macro2"))]
pub extern crate proc_macro;
#[cfg(feature = "proc-macro2")]
pub use proc_macro2 as proc_macro;

pub mod error;
pub mod parse;

pub use parsyng_quote::ToTokens;

pub use parsyng_quote;

#[macro_export]
macro_rules! parsyng {
    ($($t:tt)*) => {{
        use $crate::parsyng_quote;
        $crate::parsyng_quote::parsyng! {
        $($t)*
    } }};
}
