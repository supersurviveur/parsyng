use simple_use_macros::simple_macro;

fn main() {
    println!("Hello, world!");

    simple_macro! {
        match, match, 8
        pub(in ::a::test) struct Foo {
            field: core::u8,
            field2: Foo<u8>
        }
        7
    };
}
