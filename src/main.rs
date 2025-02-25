mod fibonacci;
use fibonacci::fib;
fn main() {
    let result = fib(100);

    println!("The 100th fibonacci number is: {}", result);

}