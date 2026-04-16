use std::{borrow::Cow, rc::Rc, sync::Arc};

use crate::{
    ToTokens,
    proc_macro::{Ident, Literal, Span, TokenStream},
};

macro_rules! literal_impls {
    (@deref $($ty:ty => $f:ident,)*) => {
        $(
            impl ToTokens for $ty {
                fn to_tokens(&self, tokens: &mut TokenStream) {
                    tokens.extend(core::iter::once(Literal::$f(*self)));
                }
            }
        )*
    };
    (@copied $($ty:ty => $f:ident,)*) => {
        $(
            impl ToTokens for $ty {
                fn to_tokens(&self, tokens: &mut TokenStream) {
                    tokens.extend(core::iter::once(Literal::$f(self)));
                }
            }
        )*
    };
}

literal_impls! {
    @deref
    u8 => u8_suffixed,
    u16 => u16_suffixed,
    u32 => u32_suffixed,
    u64 => u64_suffixed,
    u128 => u128_suffixed,
    usize => usize_suffixed,

    i8 => i8_suffixed,
    i16 => i16_suffixed,
    i32 => i32_suffixed,
    i64 => i64_suffixed,
    i128 => i128_suffixed,
    isize => isize_suffixed,

    f32 => f32_suffixed,
    f64 => f64_suffixed,

    char => character,
}

literal_impls! {
    @copied
    str => string,
    String => string,

    std::ffi::CStr => c_string,
    std::ffi::CString => c_string,
}

impl ToTokens for bool {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(core::iter::once(Ident::new(
            if *self { "true" } else { "false" },
            Span::call_site(),
        )));
    }
}

macro_rules! proc_macro_impls {
    ($($ty:ty, )*) => {
        $(
            impl ToTokens for $ty {
                fn to_tokens(&self, tokens: &mut TokenStream) {
                    tokens.extend(core::iter::once(self.clone()));
                }
            }
        )*
    };
}

proc_macro_impls! {
    crate::proc_macro::Ident,
    crate::proc_macro::Punct,
    crate::proc_macro::Literal,
    crate::proc_macro::Group,
    crate::proc_macro::TokenTree,
    crate::proc_macro::TokenStream,
}

impl<T: ToTokens> ToTokens for Option<T> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        if let Some(val) = self {
            val.to_tokens(tokens);
        }
    }
}

impl<T: ToTokens + ToOwned> ToTokens for Cow<'_, T> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        (**self).to_tokens(tokens);
    }
}

impl<T: ToTokens> ToTokens for &T {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        (**self).to_tokens(tokens);
    }
}

impl<T: ToTokens> ToTokens for &mut T {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        (**self).to_tokens(tokens);
    }
}

impl<T: ToTokens> ToTokens for Box<T> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        (**self).to_tokens(tokens);
    }
}

impl<T: ToTokens> ToTokens for Rc<T> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        (**self).to_tokens(tokens);
    }
}

impl<T: ToTokens> ToTokens for Arc<T> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        (**self).to_tokens(tokens);
    }
}
