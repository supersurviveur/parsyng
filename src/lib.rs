#![deny(
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo,
    // rustdoc::all,
    rustdoc::redundant_explicit_links,
    invalid_doc_attributes,
    unused_doc_comments,
    // missing_docs
)]

pub use parsyng_core::*;

pub use parsyng_proc_macros::proc_macro_attribute_ as proc_macro_attribute;
pub use parsyng_proc_macros::proc_macro_derive_ as proc_macro_derive;
pub use parsyng_proc_macros::{Parse, ToTokens, proc_macro};
