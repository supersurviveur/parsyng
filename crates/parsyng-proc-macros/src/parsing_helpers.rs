macro_rules! parse_ident {
    ($stream:ident) => {
        match $stream.next() {
            Some(proc_macro::TokenTree::Ident(ident)) => ident,
            other => {
                expect_error!("identifier", other)
            }
        }
    };
}

macro_rules! expect_error {
    ($expected:literal, $received:ident) => {{
        let error = if let Some(tt) = $received {
            (
                format!(concat!("expected ", $expected, ", found `{}`"), tt).into(),
                tt.span(),
            )
        } else {
            (
                concat!("expected ", $expected, ", found `<eof>`").into(),
                Span::call_site(),
            )
        };
        return Err(error);
    }};
}
