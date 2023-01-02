#![allow(clippy::all, unused)]
use typed_builder::TypedBuilder;

/// Using typed-builder from https://github.com/idanarye/rust-typed-builder
fn main() {
    // Using optional fields
    // Note: `setter(strip_option)` removes the need for us to wrap the values in `Some(...)`
    let u = User::builder()
        .age(100)
        .active(true)
        .name("Foo".into())
        .build();
    println!("{:?}", u);

    // Using only the required fields
    let u = User::builder().name("Bar".into()).build();
    println!("{:?}", u);

    // Does not compile since a required field is missing"
    // let u = User::builder().build();
}

#[derive(TypedBuilder, Debug)]
struct User {
    name: String,
    #[builder(default, setter(strip_option))]
    age: Option<u8>,
    #[builder(default, setter(strip_option))]
    email: Option<String>,
    #[builder(default = false)]
    active: bool,
}
