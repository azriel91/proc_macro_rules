use function_like::{ensure_empty, function_like, rename};

function_like!("these are ignored", abc, struct Thing;,,,,,,, {} []);
ensure_empty!();

#[test]
fn test_hello() {
    assert_eq!("rara", hello());
}

rename! {
    pub fn name() -> &'static str { "tom" }

    // pub fn name<T>() -> &'static str where T: Clone {
    //     // hello
    //     "tom"
    // }
}

rename! {
    pub fn name() -> &'static str { "azriel" }
}

#[test]
fn tom_test() {
    assert_eq!("tom", tom());
    // assert_eq!("tom", tom::<()>());
}

#[test]
fn azriel_test() {
    assert_eq!("azriel", azriel());
}
