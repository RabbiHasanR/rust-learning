use std::result;

pub fn gcd(a: u32, b: u32) -> u32 {
    if b > 0 {
        return gcd(b, a % b);
    } else {
        return a;
    }
}

pub fn pass_args_as_reference() {
    let my_string = String::from("Hello, Rust!");
    let length = calculate_length(&my_string);
    println!("The length of '{my_string}' is {length}");
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

pub fn mutable_ref() {
    let mut num = 10;
    change_value(&mut num);
    println!("New value is: {num}");
}

fn change_value(val: &mut i32) {
    *val += 1;
}

pub fn return_multiple_value() {
    let (x, y) = swap(5, 10);
    println!("Swapped: x = {x}, y = {y}");
}

fn swap(a: i32, b: i32) -> (i32, i32) {
    (b, a)
}

pub fn generic_example() {
    print_anything(42);
    print_anything("Hello Rust");
}

fn print_anything<T: std::fmt::Debug>(item: T) {
    println!("{:?}", item);
}

pub fn passing_func_as_arguments() {
    let result = apply(square, 5);
    println!("The square is: {}", result);
}

fn apply(f: fn(i32) -> i32, value: i32) -> i32 {
    f(value)
}

fn square(x: i32) -> i32 {
    x * x
}

pub fn return_function_example() {
    let f = return_function();
    let result = f(6);
    println!("The square is: {result}");
}

fn return_function() -> fn(i32) -> i32 {
    square
}

pub fn closures_example() {
    let add = |x: i32, y: i32| -> i32 { x + y };
    let result = add(4, 5);
    println!("4 + 5 = {result}");
}

// recursion example

pub fn recursion_example() {
    let result = factorial(5);
    println!("Factorial of 5 is: {result}");
}
fn factorial(n: u32) -> u32 {
    if n == 0 {
        1
    } else {
        n * factorial(n - 1)
    }
}
