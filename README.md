# hado

Monadic haskell-like expressions brought to rust via the `hado!` macro

## What?

A little macro for writing haskell-like do expressions without too much ceremony

## Why?

Rust is a very explicit language when it comes to errors (via Option and Result
types) but it can get cumbersome to handle all of them, so this library brings
the composable monad pattern from haskell like languages.

Let's show an example: We will try to do simple math and compose possible
failures.

First we define our return type, this type will represent the failure or success
of the functions.

```rust
type MathResult = Option<f64>;
```

- Division: if the divisor is 0 we fail

```rust
  fn div(x: f64, y: f64) -> MathResult {
      if y == 0.0 {
          None
      } else {
          Some(x / y)
      }
  }
```

- Square root: if we get a negative number, we fail

```rust
  fn sqrt(x: f64) -> MathResult {
      if x < 0.0 {
          None
      } else {
          Some(x.sqrt())
      }
  }
```

- Logarithm: again if we get a negative number, we fail

```rust
  fn ln(x: f64) -> MathResult {
      if x < 0.0 {
          None
      } else {
          Some(x.ln())
      }
  }
```

Now we want to get two numbers, divide them, get the sqrt of the result and then
get the logarithm of the last result

## The naive way

```rust
  fn op(x: f64, y: f64) -> MathResult {
      let ratio = div(x, y);
      if ratio == None {
          return None
      };
      let ln = ln(ratio.unwrap());
      if ln == None {
          return None
      };
      return sqrt(ln.unwrap())
  }
```

Even though this code works it's hard to scale, and it isn't idiomatic rust, it
looks more like code you'd see in Java where `None` is `NULL`.

## The better way

```rust
  fn op(x: f64, y: f64) -> MathResult {
      match div(x, y) {
          None => None,
          Ok(ratio) => match ln(ratio) {
              None => None,
              Ok(ln) => sqrt(ln),
          },
      }
  }
```

This example is more rustic but it still looks like too much noise, and still
it's very hard to scale

## The FP way

```rust
  fn op(x: f64, y: f64) -> MathResult {
      div(x, y).and_then(|ratio|
      ln(ratio).and_then(|ln|
      sqrt(ln)))
  }
```

This way look almost like the special thing that we want to do but those
`and_then` and closures seem unnecessary

## The hado macro way

```rust
  fn op(x: f64, y: f64) -> MathResult {
      hado! {
          ratio <- div(x, y);
          ln <- ln(ratio);
          sqrt(ln)
      }
  }
```


Here we have a very obvious way of declaring out intent without no sign of error
handling of any kind, we needed to add a trait `Monad` to Option (which is
already defined by default in this library)

## Error type agnostic

Now some more fancy stuff, you may be thinking `but what about the try! macro,
it would certainly make things better, right?` and my answer would be yes but
the try macro _only_ works on the `Result` type so there is no way of changing
the type of the error (or use it with our Option based functions).


But now we need to know what was the error that made of computation fail. So we
change the `MathResult` alias to be a Result of f64 or a custom type `MathError`

```rust
  #[derive(Debug)]
  pub enum MathError {
      DivisionByZero,
      NegativeLogarithm,
      NegativeSquareRoot,
  }

  type MathResult = Result<f64, MathError>;
```

So now we need to change each function because now all the `None` and `Some`
constructors are a type error

For example div turns into

```rust
  fn div(x: f64, y: f64) -> MathResult {
      if y == 0.0 {
          Err(MathError::DivisionByZero)
      } else {
          Ok(x / y)
      }
  }
```

note that the only changes are:

- ~~None~~ `Err(MathError::DivisionByZero)`
- ~~Some~~ `Ok (x / y)`

and now we check out the op function for each implementation:

* *Naive way*: Change all the constructors, the failure checker, luckily rust's
     type inference saves us from changing too many type declarations.
* *Better way*: Slightly better, same constructors, but failure checkers are
     replaced by match statements but still very verbose.
* *FP way*: a lot better, the only worry we have is that the new error type has
     some sort of ~and_then~ and everything else should work.
* *hado way*: Similar to the last, but now there is nothing to change, the
     computation has no concern over the failure framework.

# How can I use it?

The macro can do some basic stuff based on a trait defined inside the crate
*Monad* which has implementations for Option and Result by default

Let's take some examples from the rust book and rust by example and translate
them into hado format.

## [Example from Result type reference](https://doc.rust-lang.org/std/result/#the-try-macro)

Here is the original try based error handling with early returns

```rust
  fn write_info(info: &str) -> io::Result<()> {
      let mut file = try!(File::create("my_best_friends.txt"));
      println!("file created");
      try!(file.write_all(format!("rating: {}\n", info.rating).as_bytes()));
      Ok(())
  }
```

And here is the hado based

```rust
  fn hado_write_info(string: &str) -> io::Result<()> {
      hado!{
          mut file <- File::create("my_best_friends.txt");
          println!("file created");
          file.write_all(format!("string: {}\n", string).as_bytes())
      }
  }
```

Note that the ign keyword is special, it means that the inner value is discarded
but in the case of failure the whole expressions will short circuit into that
error. Since there is no arrow (`<-`) the return of the `println` is completely
discarded so you can have any non failing statements in there (including `let`
and `use`)

## Multi parameter constructor

let's say we have a Foo struct that has a new method

```rust
  fn new(a: i32, b: f64, s: String) -> Foo {
      Foo {
          a: i32,
          b: f64,
          s: String
      }
  }
```

Create a Option constructor from a normal constructor with minimal hassle

```rust
  fn opt_new(a: Option<i32>, b: Option<f64>, s: Option<String>) -> Option<Foo> {
      a <- a;
      b <- b;
      s <- s;
      ret new(a, b, s)
  }
```

Note that only by changing the type signature you can change Option to any other
monadic type.

You can also do some custom validation without much boilerplate

```rust
  fn opt_new(a: i32, b: f64, s: String) -> Option<Foo> {
      a <- validate_a(a);
      b <- validate_b(b);
      s <- validate_s(s);
      ret new(a, b, s)
  }
```
