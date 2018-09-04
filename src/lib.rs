//! # Number As
//! **Use** the trait `NumberAs` in the current scope to let all primitive number types have a `number_as` method.
//!
//! ```
//! extern crate number_as;
//!
//! use number_as::NumberAs;
//!
//! let a: i32 = 2u16.number_as();
//!
//! assert_eq!(2i32, a);
//!
//! assert_eq!(2i32, 2u16.number_as());
//!
//! assert_eq!(20i32, 20.6.number_as());
//! ```
//!
//! All implements for the `NumberAs` trait can be considered as a whole **Primitive Number**. If you want to design an **add** function, which accepts all kinds of primitive numbers,
//! you can use the `Number` trait as a trait bound.
//!
//! ```
//! extern crate number_as;
//!
//! use number_as::{Number, NumberAs};
//!
//! fn add<T, J, K>(a: T, b: J) -> K where T: Number, J: Number, K: Number, i128: number_as::NumberAs<K> {
//!     let a: i128 = a.number_as();
//!     let b: i128 = b.number_as();
//!
//!     let c = a + b;
//!
//!     c.number_as()
//! }
//!
//! assert_eq!(40i32, add(5u8, 35i16));
//! ```

pub trait NumberAs<T>: Sized {
    fn number_as(self) -> T;
}

macro_rules! number_as_impl {
    ( $from:ty, $to:ty ) => {
        impl NumberAs<$to> for $from {
            fn number_as(self) -> $to {
                self as $to
            }
        }
    }
}

macro_rules! number_as_batch {
    ( $from:ty ) => {
        number_as_impl!($from, u8);
        number_as_impl!($from, u16);
        number_as_impl!($from, u32);
        number_as_impl!($from, u64);
        number_as_impl!($from, u128);
        number_as_impl!($from, usize);
        number_as_impl!($from, i8);
        number_as_impl!($from, i16);
        number_as_impl!($from, i32);
        number_as_impl!($from, i64);
        number_as_impl!($from, i128);
        number_as_impl!($from, isize);
        number_as_impl!($from, f32);
        number_as_impl!($from, f64);
    }
}

number_as_batch!(u8);
number_as_batch!(u16);
number_as_batch!(u32);
number_as_batch!(u64);
number_as_batch!(u128);
number_as_batch!(usize);

number_as_batch!(i8);
number_as_batch!(i16);
number_as_batch!(i32);
number_as_batch!(i64);
number_as_batch!(i128);
number_as_batch!(isize);

number_as_batch!(f32);
number_as_batch!(f64);

pub trait Number: NumberAs<u8> + NumberAs<u16> + NumberAs<u32> + NumberAs<u64> + NumberAs<u128> + NumberAs<usize>
+ NumberAs<i8> + NumberAs<i16> + NumberAs<i32> + NumberAs<i64> + NumberAs<i128> + NumberAs<isize>
+ NumberAs<f32> + NumberAs<f64> {}

macro_rules! number_impl {
    ( $t:ty ) => {
        impl Number for $t { }
    }
}

number_impl!(u8);
number_impl!(u16);
number_impl!(u32);
number_impl!(u64);
number_impl!(u128);
number_impl!(usize);

number_impl!(i8);
number_impl!(i16);
number_impl!(i32);
number_impl!(i64);
number_impl!(i128);
number_impl!(isize);

number_impl!(f32);
number_impl!(f64);

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! number_as_batch_test {
        ( $from:expr ) => {
            let _a: u8 = $from.number_as();
            let _a: u16 = $from.number_as();
            let _a: u32 = $from.number_as();
            let _a: u64 = $from.number_as();
            let _a: u128 = $from.number_as();
            let _a: usize = $from.number_as();
            let _a: i8 = $from.number_as();
            let _a: i16 = $from.number_as();
            let _a: i32 = $from.number_as();
            let _a: i64 = $from.number_as();
            let _a: i128 = $from.number_as();
            let _a: isize = $from.number_as();
            let _a: f32 = $from.number_as();
            let _a: f64 = $from.number_as();
        }
    }

    #[test]
    fn it_works() {
        number_as_batch_test!(0u8);
        number_as_batch_test!(0u16);
        number_as_batch_test!(0u32);
        number_as_batch_test!(0u64);
        number_as_batch_test!(0u128);
        number_as_batch_test!(0usize);
        number_as_batch_test!(0i8);
        number_as_batch_test!(0i16);
        number_as_batch_test!(0i32);
        number_as_batch_test!(0i64);
        number_as_batch_test!(0i128);
        number_as_batch_test!(0isize);
        number_as_batch_test!(0f32);
        number_as_batch_test!(0f64);
    }
}