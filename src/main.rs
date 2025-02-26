mod fibonacci;
use fibonacci::fib;
fn main() {
    let result = fib(-185);

    println!("The 100th fibonacci number is: {}", result);

}