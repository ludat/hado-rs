#![feature(type_ascription)]

// TODO core of this crate
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

trait Monad<O> {
    type Inner;
    fn bind<F>(t: Self, f: F) -> O where F: Fn(Self::Inner) -> O ;
    fn ret(Self::Inner) -> Self;
}

impl<T, O> Monad<Option<O>> for Option<T> {
    type Inner = T;
    fn bind<F>(t: Option<T>, f: F) -> Option<O>
        where F: Fn(T) -> Option<O> {
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
    fn bind<F>(t: Result<T, E>, f: F) -> Result<O, E>
        where F: Fn(T) -> Result<O, E> {
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
    fn bind<F>(t: Self, f: F) -> Vec<O>
        where F: Fn(T) -> Vec<O> {
        let mut acc: Vec<O> = Vec::new();
        for v in t {
            acc.append(&mut f(v))
        }
        acc
    }
    fn ret(inner: T) -> Vec<T> {
        vec![inner]
    }
}

#[cfg(test)]
mod tests {
    use super::Monad;
    #[test] fn hado() {
        assert_eq!(
            Some(3),
            hado!(
                x <- Some(1);
                y <- Some(2);
                Some(x + y)
            )
        );

        assert_eq!(
            Some(3),
            hado!(
                x <- Some(1);
                println!("yey");
                y <- Some(2);
                Some(x + y)
            )
        );

        assert_eq!(
            vec![0, 0, 1, 1, 2, 2]: Vec<u32>,
            hado!(
                a <- vec![0, 1, 2];
                b <- vec![a, a];
                ret(Vec<u32>) b
            )
        );
        assert_eq!(
            hado!(
                a <- vec![0, 1, 2];
                b <- vec![a, a];
                ret(Vec<u32>) b
            ),
            hado!(
                a <- vec![0, 1, 2];
                vec![a, a]
            )
        );
        assert_eq!(
            hado!(
                ign <- vec![0, 1, 2];
                let b = 7;
                ret(Vec<u32>) b
            ),
            hado!(
                ign <- vec![0, 1, 2];
                vec![7]
            )
        );
    }
    #[test] fn option_monad() {
        assert_eq!(
            Some(0),
            Monad::bind(Some(1), |x| Some(x*0))
        );
        assert_eq!(
            None,
            Monad::bind(None, |x: u32| Some(x*0))
        );
    }
    #[test] fn result_monad() {
        assert_eq!(
            Ok(0),
            Monad::<Result<u32, &'static str>>::bind(Ok(1), |x: u32| Ok(x*0))
        );
        assert_eq!(
            Err("failed"),
            Monad::bind(Err("failed"), |x: u32| Ok(x*0))
        );
    }
    #[test] fn list_monad() {
        assert_eq!(
            Vec::new(): Vec<u32>,
            Monad::<Vec<u32>>::bind(Vec::new(), |_: u32| vec![0, 0])
        );
        assert_eq!(
            vec![0, 0, 0, 0]: Vec<u32>,
            Monad::<Vec<u32>>::bind(vec![0, 0], |_: u32| vec![0, 0])
        );
        assert_eq!(
            vec![0, 0, 1, 1, 2, 2]: Vec<u32>,
            Monad::<Vec<u32>>::bind(vec![0, 1, 2], |x: u32| vec![x, x])
        );
    }
}
