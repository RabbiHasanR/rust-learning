fn fib(n: u32) -> u32 {
    if n < 2 {
        return n;
    } else {
        return fib(n - 1) + fib(n - 2);
    }
}

pub fn fibonacci(n: u32){
    let result = fib(n);
    println!("Fibonacci of {n} is {result}");
}