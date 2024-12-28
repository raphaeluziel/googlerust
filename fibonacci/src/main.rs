fn fib(n: u64) -> u64 {
    if n < 2 {
        // The base case.
        n
    } else {
        // The recursive case.
        fib(n - 1) + fib(n - 2)
    }
}

fn main() {
    let n = 50;
    println!("fib({n}) = {}", fib(n));
}