# enum_inject

[![Version info](https://img.shields.io/crates/v/enum_inject.svg)](https://crates.io/crates/enum_inject)
[![Downloads](https://img.shields.io/crates/d/enum_inject.svg?style=flat-square)](https://crates.io/crates/enum_inject)
[![docs](https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square)](https://docs.rs/enum_inject)
[![dependency status](https://deps.rs/crate/enum_inject/0.1.2/status.svg)](https://deps.rs/crate/enum_inject)

# example:
```rust
    use enum_inject::{enum_injector, EnumInjector};
    use display_enum::Display;
    #[derive(EnumInjector)]
    #[enum_injector({"args":[{"prefix":"A","suffix":"B","compute":{"Mul":1000}},{"prefix":"AA","suffix":"BB","compute":{"Mul":3000}}],"derives":["#[repr(i32)]","#[derive(Display)]"]})]
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
```