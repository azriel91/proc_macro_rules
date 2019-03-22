use function_like::{function_like, rename};

function_like!("these are ignored", abc, struct Thing;,,,,,,, {} []);

#[test]
fn test_hello() {
    assert_eq!("rara", hello());
}

rename! {
    pub fn name<T>() -> &'static str where T: Clone { "tom" }
}

rename! {
    pub fn name() -> &'static str { "azriel" }
}

#[test]
fn tom_test() {
    assert_eq!("tom", tom::<()>());
}

#[test]
fn azriel_test() {
    assert_eq!("azriel", azriel());
}
