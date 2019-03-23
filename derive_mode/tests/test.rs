// === derive new === //
//
// cargo expand --test test derive_new

pub mod derive_new {
    use derive_mode::new;

    #[derive(new)]
    pub struct MyStruct;

    #[test]
    fn new() {
        let _ = MyStruct::new();
    }
}

// === derive Foo trait impl === //
//
// cargo expand --test test toufoo

/// The `Foo` trait.
trait Foo {
    fn foo() -> u32;
}

mod toufoo {
    use super::Foo; // Trait must be in scope, or use the full path in the macro.

    #[derive(derive_mode::Foo)]
    struct Tou;

    #[test]
    fn tou_foo() {
        assert_eq!(42, Tou::foo());
        assert_eq!(42, <Tou as crate::Foo>::foo());
    }
}

// === show attr === //
//
// cargo expand --test test show_attr
//
// How To Use:
//
// 1. Change the `show_attr` attribute to any of the attribute names.
// 2. Run `cargo expand --test test show_attr`
// 3. The generated function will contain the attribute if it was passed in to the function.

pub mod show_attr {
    use derive_mode::{ShowAttribute, SomeoneElsesDerive};

    /// Hello this is the "doc" `Meta::NameValue` attribute
    ///
    /// With multiple lines.
    #[derive(ShowAttribute, SomeoneElsesDerive, Debug)]
    #[show_attr("derive")]
    #[random(nested("a", "b"))]
    #[someone_elses_attr]
    #[warn(missing_debug_implementations, unknown_lints)]
    pub struct StructWithAttributes;
}
