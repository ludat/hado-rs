extern crate hado;

use hado::Monad;

#[test] fn simple_map() {
    assert_eq!(
        Ok(0),
        Monad::<Result<u32, &'static str>>::bind(Ok(1), |x: u32| Ok(x*0))
    );
}
#[test] fn fail_fast() {
    assert_eq!(
        Err("failed"),
        Monad::bind(Err("failed"), |x: u32| Ok(x*0))
    );
}
