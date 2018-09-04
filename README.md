Validators
====================

[![Build Status](https://travis-ci.org/magiclen/number-as.svg?branch=master)](https://travis-ci.org/magiclen/number-as)

**Use** the trait `NumberAs` in the current scope to let all primitive number types have a `number_as` method.

```rust
extern crate number_as;

use number_as::NumberAs;

let a: i32 = 2u16.number_as();

assert_eq!(2i32, a);

assert_eq!(2i32, 2u16.number_as());

assert_eq!(20i32, 20.6.number_as());
```

All implements for the `NumberAs` trait can be considered as a whole **Primitive Number**. If you want to design an **add** function, which accepts all kinds of primitive numbers,
you can use the `Number` trait as a trait bound.

```rust
extern crate number_as;

use number_as::{Number, NumberAs};

fn add<T, J, K>(a: T, b: J) -> K where T: Number, J: Number, K: Number, i128: number_as::NumberAs<K> {
    let a: i128 = a.number_as();
    let b: i128 = b.number_as();

    let c = a + b;

    c.number_as()
}

assert_eq!(40i32, add(5u8, 35i16));
```

## Crates.io

https://crates.io/crates/number-as

## Documentation

https://docs.rs/number-as

## License

[MIT](LICENSE)