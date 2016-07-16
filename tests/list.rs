#[macro_use]
extern crate hado;

use hado::Monad;

#[test] fn empty_list_is_empty_list() {
    assert_eq!(
        { let t: Vec<u32> = vec![]; t },
        Monad::<Vec<u32>>::bind(Vec::new(), |_: u32| vec![0, 0])
    );
}
#[test] fn two_lists_multiply() {
    assert_eq!(
        { let t: Vec<u32> = vec![0, 0, 0, 0, 0, 0]; t },
        Monad::<Vec<u32>>::bind(vec![0, 0, 0], |_: u32| vec![0, 0])
    );
}
#[test] fn duplicate_list() {
    assert_eq!(
        { let t: Vec<u32> = vec![0, 0, 1, 1, 2, 2]; t },
        Monad::<Vec<u32>>::bind(vec![0, 1, 2], |x: u32| vec![x, x])
    );
}
