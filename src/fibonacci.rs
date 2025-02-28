pub fn fib(n: i128) -> i128 {

    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }
    if n < 0 {
        return fib(n + 2) - fib(n + 1);
    }
    let mut prev = 0;
    let mut curr = 1;
    for _ in 0..n {
        let next = prev + curr;
        prev = curr;
        curr = next;
    }
    prev
}