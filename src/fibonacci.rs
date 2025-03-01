use num_bigint::{BigInt, ToBigInt};

pub fn fib(c: u128) -> BigInt {
    match c {
        0 => 0.to_bigint().expect("Invalid input"),
        1 => 1.to_bigint().expect("Invalid input"),
        _ => fib(c - 1) + fib(c - 2),
    }
}