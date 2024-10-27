use enum_inject::{enum_injector, EnumInjector};
use display_enum::Display;
#[derive(EnumInjector)]
#[enum_injector({"args":[{"prefix":"A","suffix":"B","compute":{"Mul":1000}},{"prefix":"AA","suffix":"BB","compute":{"Mul":3000}}],"derives":["#[repr(i32)]","#[derive(Display)]"]})]
enum Foo {
    #[skip]
    Test,
    B,
    C
}

#[test]
fn test() {
    println!("{}", Foo::ABB);
    println!("{}", Foo::AABBB as i32);
}