use std::result;

type kilometers = u32;

pub fn type_alias_example() {
    let distance: kilometers = 100;
    println!("Distance: {} km", distance);
}


// aliasing complex types

type ComplexResult = Result<Vec<(i32, String)>, String>;

fn get_result() -> ComplexResult {
    Ok(vec![(42, String::from("Answer"))])
}

pub fn type_alias_example_two() {
    let result = get_result();

    match result {
        Ok(value) => println!("{value:?}"),
        Err(e) => println!("error: {e}"),
    }
}



// aliasing function pointer types

type MathOp = fn(i32, i32) -> i32;

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn apply_operation(op: MathOp, a: i32, b: i32) -> i32 {
    op(a, b)
}

pub fn type_alias_example_three() {
    let result = apply_operation(add, 5, 3);
    println!("Result: {}", result);
}


// aliasing with lifetimes and generics

type ResultWithLifetime<'a> = Result<&'a i32, &'a str>;

fn get_referenc<'a>(arr: &'a [i32]) -> ResultWithLifetime<'a> {
    arr.get(0).ok_or("Empty array")
}

pub fn type_alias_example_four() {
    let result = get_referenc(&[1,2,3,4,5]);
    match result {
        Ok(value) => println!("value: {}", value),
        Err(e) => println!("error: {}", e)
    }
}


// type aliases for custom types

struct Point {
    x: i32,
    y: i32,
}

type Point2D = Point;

pub fn type_alias_example_five() {
    let point: Point2D = Point { x: 10, y: 20 };
    println!("Point: ({}, {})", point.x, point.y);
}


