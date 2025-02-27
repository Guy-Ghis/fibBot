use crate::fibonacci::fib;

pub fn fibonacci_vector(numbers: Vec<i128>) -> Vec<i128> {
    numbers.into_iter().map(|n| fib(n.try_into().unwrap()) as i128).collect()
}