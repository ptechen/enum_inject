use enum_inject::{enum_injector, EnumInjector};
use display_enum::Display;
#[derive(EnumInjector)]
#[enum_injector({"args":[{"prefix":"A","suffix":"B","compute":{"Mul":1000}},{"prefix":"AA","suffix":"BB","compute":{"Mul":3000}}],"derives":["#[repr(i32)]","#[derive(Display)]"]})]
#[derive(Display)]
enum Foo {
    #[skip]
    Test,
    #[sync_attr]
    B,
    C
}

#[test]
fn test() {
    assert_eq!("ABB", Foo::ABB.to_string());
    assert_eq!(3000, Foo::AABBB as i32);
}