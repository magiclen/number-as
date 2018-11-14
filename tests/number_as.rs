extern crate number_as;

use number_as::NumberAs;

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