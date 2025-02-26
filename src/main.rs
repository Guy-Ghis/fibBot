mod fibonacci;
use fibonacci::fib;
fn main() {
    let result = fib(20);

    println!("The 20th fibonacci number is: {}", result);

}