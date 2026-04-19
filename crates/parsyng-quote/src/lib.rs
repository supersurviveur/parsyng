#![deny(clippy::pedantic, clippy::all)]

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

#[doc(hidden)]
pub mod __private;

pub mod impls;

pub use parsyng_quote_macros::{parsyng, parsyng_spanned};

#[macro_export]
macro_rules! format_ident {
    ($($args:tt),*) => {
        $crate::proc_macro::Ident::new(&format!($($args,)*), $crate::proc_macro::Span::call_site())
    };
}

pub trait ToTokens {
    fn to_tokens(&self, tokens: &mut crate::proc_macro::TokenStream);
}
