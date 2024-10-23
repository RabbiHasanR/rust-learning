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