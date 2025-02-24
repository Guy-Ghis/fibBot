// recursive approach

// fn fib(n: u32) -> u32 {
//     if n <= 1 {
//         return n;
//     }
//     fib(n - 1) + fib(n - 2)
// }

// Iterative approach

fn fib(n: u32) -> u32 {
    let mut prev = 0;
    let mut curr = 1;
    for _ in 0..n {
        let next = prev + curr;
        prev = curr;
        curr = next;
    }
    prev
}