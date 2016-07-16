#[macro_use]
extern crate hado;

#[test] fn main() {
    assert_eq!(
        Some(3),
        hado! {
            x <- Some(1);
            y <- Some(2);
            Some(x + y)
        }
    );
    assert_eq!(
        Some(3),
        hado! {
            x <- Some(1);
            println!("x: {:?}", x);
            y <- Some(2);
            Some(x + y)
        }
    );
    assert_eq!(
        { let t: Vec<u32> = vec![0, 0, 1, 1, 2, 2]; t },
        hado! {
            a <- vec![0, 1, 2];
            println!("a: {:?}", a);
            b <- vec![a, a];
            ret(Vec<u32>) b
        }
    );
    assert_eq!(
        hado! {
            a <- vec![0, 1, 2];
            b <- vec![a, a];
            ret(Vec<u32>) b
        },
        hado! {
            a <- vec![0, 1, 2];
            vec![a, a]
        }
    );
    assert_eq!(
        hado! {
            ign <- vec![0, 1, 2];
            let b = 7;
            ret(Vec<u32>) b
        },
        hado! {
            ign <- vec![0, 1, 2];
            vec![7]
        }
    );
}

#[test] fn file() {
    use std::fs::File;
    use std::io::prelude::*;
    use std::io;

    #[derive(Clone, PartialEq)]
    struct Info {
        name: String,
        age: i32,
        rating: i32,
    }


    fn monadic_write_info(info: &Info) -> io::Result<()> {
        hado!{
            mut file <- File::create("my_best_friends.txt");
            ign <- file.write_all(format!("name: {}\n", info.name).as_bytes());
            ign <- file.write_all(format!("age: {}\n", info.age).as_bytes());
            file.write_all(format!("rating: {}\n", info.rating).as_bytes())
        }
    }

    println!("result: {:?}",
        monadic_write_info(&Info {
            name: "Lucas".to_string(),
            age: 20,
            rating: 15,
        })
    )
}

#[test] fn division() {
    // Mathematical "errors" we want to catch
    #[derive(Debug, PartialEq)]
    pub enum MathError {
        DivisionByZero,
        NegativeLogarithm,
        NegativeSquareRoot,
    }

    type MathResult = Result<f64, MathError>;
    // type MathResult = Option<f64>;

    fn div(x: f64, y: f64) -> MathResult {
        if y == 0.0 {
            Err(MathError::DivisionByZero)
        } else {
            Ok(x / y)
        }
    }
    fn sqrt(x: f64) -> MathResult {
        if x < 0.0 {
            Err(MathError::NegativeSquareRoot)
        } else {
            Ok(x.sqrt())
        }
    }
    fn ln(x: f64) -> MathResult {
        if x < 0.0 {
            Err(MathError::NegativeLogarithm)
        } else {
            Ok(x.ln())
        }
    }
    fn op(x: f64, y: f64) -> MathResult {
        hado! {
            ratio <- div(x, y);
            println!("{:?}", ratio);
            ln <- ln(ratio);
            println!("{:?}", ln);
            sqrt(ln)
        }
    }

    assert_eq!(
        op(1.0, 10.0),
        Err(MathError::NegativeSquareRoot)
    );
}
