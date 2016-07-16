#[macro_use]
extern crate hado;

use hado::Monad;

#[test] fn simple_map() {
    assert_eq!(
        Some(0),
        Monad::bind(Some(1), |x| Some(x*0))
    );
}
#[test] fn two_level_deep() {
    assert_eq!(
        Some(3),
        Monad::bind(Some(1), |x| Monad::bind(Some(2), |y| Some(x + y)))
    );
}
#[test] fn if_none_ignore_function() {
    assert_eq!(
        None,
        Monad::bind(None, |x: u32| Some(x*0))
    );
}
