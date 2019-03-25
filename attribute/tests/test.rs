// === noop === //
//
// cargo expand --test test noop

pub mod noop {
    use attribute::noop;

    // The commented out attributes need
    // `#![feature(proc_macro_hygiene)]` on the crate.

    #[noop]
    pub struct Struct;

    #[noop]
    pub enum Enum {}

    #[noop]
    pub type Type = ();

    #[noop]
    pub trait Trait {}

    #[noop]
    pub fn make_string(arg: u32) -> String {
        // #[noop]
        let string = format!("{:?}", arg);

        // #[noop]
        string
    }

    pub fn generic<
        // #[noop]
        T,
    >(
        // #[noop]
        _: T,
    ) -> T
    // #[noop]
    where
        // #[noop]
        T: Default,
    {
        Default::default()
    }

    // #[noop]
    mod inner {}
}

#[test]
fn test_make_string() {
    assert_eq!("123", noop::make_string(123).as_str());
}

// === remove === //
//
// cargo expand --test test remove

pub mod remove {
    use attribute::remove;

    // Commenting the following would cause rustc to warn that
    // `Struct` is unused.
    #[remove]
    #[derive(Debug)]
    struct Struct;
}
