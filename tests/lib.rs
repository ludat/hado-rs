#![feature(type_ascription)]

#[macro_use]
extern crate hado;

#[test] fn main() {
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
            println!("x: {:?}", x);
            y <- Some(2);
            Some(x + y)
        )
    );
    assert_eq!(
        vec![0, 0, 1, 1, 2, 2]: Vec<u32>,
        hado!(
            a <- vec![0, 1, 2];
            println!("a: {:?}", a);
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
