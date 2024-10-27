# enum_inject

[![Version info](https://img.shields.io/crates/v/enum_from_derive.svg)](https://crates.io/crates/enum_inject)
[![Downloads](https://img.shields.io/crates/d/enum_inject.svg?style=flat-square)](https://crates.io/crates/enum_inject)
[![docs](https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square)](https://docs.rs/enum_inject)
[![dependency status](https://deps.rs/crate/enum_inject/0.1.1/status.svg)](https://deps.rs/crate/enum_inject)

## example:
    
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