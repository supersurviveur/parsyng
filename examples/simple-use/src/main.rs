use simple_use_macros::simple_macro;

fn main() {
    println!("Hello, world!");

    simple_macro! {
        pub(in ::a::test) struct Foo<'a, 'b: '_ + 'static, T: 'a + Test<T> + Bar + ?Sized + (for<T> T)> where 'a: 'b, T: Add {
            field: core::primitive::u8,
            field2: Foo<T>
        }
        unsafe impl Deref for T {
            type Inner<A>:K+'a=A;
            const Q: u8 = 1;
        }
        7
    };
}
