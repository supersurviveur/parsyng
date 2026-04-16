#![deny(clippy::pedantic, clippy::all)]

#[cfg(not(feature = "proc-macro2"))]
pub extern crate proc_macro;
#[cfg(feature = "proc-macro2")]
pub use proc_macro2 as proc_macro;

#[doc(hidden)]
pub mod __private;

pub mod impls;

pub use parsyng_quote_macros::parsyng;

#[macro_export]
macro_rules! format_ident {
    ($($args:tt),*) => {
        $crate::proc_macro::Ident::new(&format!($($args,)*), $crate::proc_macro::Span::call_site())
    };
}

pub trait ToTokens {
    fn to_tokens(&self, tokens: &mut crate::proc_macro::TokenStream);
}
