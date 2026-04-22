use simple_use_macros::simple_macro;

fn main() {
    println!("Hello, world!");

    simple_macro! {
        pub(in ::a::test) struct Foo<T: Test + Bar + ?Sized + (for<T> T)> where 'a: 'b {
            field: core::primitive::u8,
            field2: Foo<T>
        }
        7
    };
}
