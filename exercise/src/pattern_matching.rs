use core::num;

use crate::tuple;

pub fn pattern_matching_example() {
    let input = 'x';

    match input {
        'q' => println!("Quitting"),
        'a' | 's' | 'w' | 'd' => println!("Moving around"),
        '0'..='9' => println!("Number input"),
        key if key.is_lowercase() => println!("Lowercase: {key}"),
        _ => println!("Something else"),
    }
}

pub fn pattern_matching_example_two() {
    let opt = Some(123);

    match opt {
        outer @ Some(inner) => {
            println!("outer: {outer:?}, inner: {inner}");
        }
        None => {}
    }
}


// matching simple values
pub fn pattern_matching_example_three() {
    let number = 5;

    match number {
        1 => println!("One"),
        2 => println!("Two"),
        3 => println!("Three"),
        _ => println!("Some other number"),
    }
}


// matching multiple patterns

pub fn pattern_matching_example_four() {
    let number = 70;

    match number {
        1 | 2 => println!("One or Two"),
        3..=5 => println!("Between Three and Five"),
        6 | 7 | 8 => println!("Six, Seven, or Eight"),
        _ => println!("Other"),
    }
}


// matching enums
enum Direction {
    North,
    South,
    East,
    West,
}

pub fn pattern_matching_example_five() {
    let direction = Direction::West;

    match direction {
        Direction::North => println!("Going North"),
        Direction::South => println!("Going South"),
        Direction::East => println!("Going East"),
        Direction::West => println!("Going West")
    }
}


// destructuring Structs and Tuples

pub fn pattern_matching_example_six() {
    let point = (10, 20);

    match point {
        (0,0) => println!("Origin"),
        (x, y) => println!("Point at ({}, {})", x, y),
    }
}

struct Point {
    x: i32,
    y: i32,
}

pub fn pattern_matching_example_seven() {
    let point = Point {x: 20, y: 30};

    match point {
        Point {x: 0, y: 0} => println!("Origin"),
        Point {x,y} => println!("Point at ({}, {})", x, y)
    }
}



// using if let for simpler pattern matching

pub fn pattern_matching_example_eight() {
    // let option_value = Some(6);
    let option_value: Option<i32> = None;


    if let Some(x) = option_value {
        println!("The value is {}", x);
    } else {
        println!("No value")
    }
}


// combining patterns with if conditions (Guards)

pub fn pattern_matching_example_nine() {
    let number = 6;

    match number {
        x if x % 2 == 0 => println!("Even"),
        _ => println!("Odd")
    }
}


// destructuring and ignoring parts of a pattern

pub fn pattern_matching_example_ten() {
    let tuple = (10,20,30);
    
    match tuple {
        (x, _, z) => println!("x: {}, z: {}", x, z)
    }
}


// using while let for loops with pattern matching

pub fn pattern_matching_example_eleven() {

    let mut numbers = vec![Some(10), Some(20), None];

    while let Some(x) = numbers.pop() {
        if let Some(value) = x {
            println!("Popped: Some({})", value);
        }
    }
}

// matching reference

pub fn pattern_matching_example_twelve() {
    let reference = &4;
    match reference {
        &val => println!("Got a value through reference: {}", val)
    }
}

pub fn pattern_matching_example_thirteen() {
    let value = 5;
    let reference = &value;

    match reference {
        ref val => println!("Got a value through reference: {}", val)
    }
}


// patterns is let binding

pub fn pattern_matching_example_fourteen() {
    let (x,y,z) = (10,20,30);

    println!("x: {}, y: {}, z: {}", x, y, z);
}