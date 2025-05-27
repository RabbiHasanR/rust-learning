mod hello;
mod variables;
mod arithmetic;
mod type_inference;
mod fibonacci;

fn main() {
    hello::say_hello();
    println!("This is a simple Rust program demonstrating module usage.");
    variables::variables();
    let result = arithmetic::interproduct(2, 3, 4);
    println!("The interproduct of 2, 3, and 4 is: {result}");

    type_inference::type_inference();
    
    let n = 20;
    fibonacci::fibonacci(n);

    println!("End of the program.");
}
