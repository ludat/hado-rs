#[macro_export]
macro_rules! hado {
    (ret ( $ty:ty ) $expr:expr) => {
        $crate::Monad::< $ty >::ret($expr)
    };
    (ret $expr:expr) => {
        $crate::Monad::ret($expr)
    };
    (ign <- $expr:expr; $($rest:tt)*) => {
        $crate::Monad::bind($expr, |_| hado!($($rest)*))
    };
    (mut $ident:ident <- $expr:expr; $($rest:tt)*) => {
        $crate::Monad::bind($expr, |mut $ident| hado!($($rest)*))
    };
    ($ident:ident <- $expr:expr; $($rest:tt)*) => {
        $crate::Monad::bind($expr, |$ident| hado!($($rest)*))
    };
    ($stmt:stmt; $($rest:tt)*) => {
        { $stmt ; hado!($($rest)*) }
    };
    ($expr:expr) => {
        $expr
    }
}

pub trait Monad<O> {
    type Inner;
    fn bind<F>(t: Self, mut f: F) -> O where F: FnMut(Self::Inner) -> O ;
    fn ret(Self::Inner) -> Self;
}

impl<T, O> Monad<Option<O>> for Option<T> {
    type Inner = T;
    fn bind<F>(t: Option<T>, mut f: F) -> Option<O>
        where F: FnMut(T) -> Option<O> {
        match t {
            Some(t) => f(t),
            None => None,
        }
    }
    fn ret(inner: T) -> Self {
        Some(inner)
    }
}

impl<T, O, E> Monad<Result<O, E>> for Result<T, E> {
    type Inner = T;
    fn bind<F>(t: Result<T, E>, mut f: F) -> Result<O, E>
        where F: FnMut(T) -> Result<O, E> {
        match t {
            Ok(t) => f(t),
            Err(e) => Err(e),
        }
    }
    fn ret(inner: T) -> Self {
        Ok(inner)
    }
}

impl<T, O> Monad<Vec<O>> for Vec<T> {
    type Inner = T;
    fn bind<F>(t: Self, mut f: F) -> Vec<O>
        where F: FnMut(T) -> Vec<O> {
        let mut acc: Vec<O> = Vec::new();
        for v in t {
            acc.append(&mut f(v));
        }
        acc
    }
    fn ret(inner: T) -> Vec<T> {
        vec![inner]
    }
}
