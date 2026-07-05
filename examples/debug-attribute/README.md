# Debug attribute example

This example shows how to use the `debug` attribute on a macro and why it can be helpfull.

When trying to compile the `example` crate with `cargo b`, rust returns a compile error, because the `faulty_macro` proc-macro returns an invalid syntax.
Sometimes, understanding why a macro returns an invalid code is hard, and if there is a syntax error in the rust parser, even `cargo expand` cannot help (You can try it on this example, it won't work).

But using the `debug` attribute on the proc-macro :

```rust
#[parsyng::proc_macro(debug)]
```

and then recompiling with `cargo b` will print on stderr the macro output. Then it's easier to spot the issue :
```text
[DEBUG] proc-macro `faulty_macro` called at examples/debug-attribute/debug-attribute-macros/src/lib.rs:4:1
let foo = 9 - 3 *;
```

The `debug-pretty` feature can be enabled in `parsyng` to pass the output through `rustfmt` when debugging complex macros.
