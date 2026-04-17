use simple_use_macros::simple_macro;

fn main() {
    println!("Hello, world! {}", simple_macro!(0b1u8));
}
