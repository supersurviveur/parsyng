pub trait LiteralExt {
    fn foo(&mut self);
}

impl LiteralExt for crate::proc_macro::Literal {
    fn foo(&mut self) {}
}
