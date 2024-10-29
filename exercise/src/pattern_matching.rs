use core::num;
use std::fmt::format;

use crate::{functions::return_function_example, tuple};

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




// patterns matching with structs

struct Foo {
    x: (u32, u32),
    y: u32,
}

#[rustfmt::skip]
pub fn pattern_matching_example_fiftten() {
    let foo = Foo {x: (10,20), y: 3};

    match foo {
        Foo {x:(a,b), y} => println!("x.0={a}, b={b}, y={y}"),
        Foo {y:e, x: i} => println!("y={e}, x={i:?}"),
        Foo {y, ..} => println!("y={y}, other fields were ignored"),
    }
}


// patterns matching with structs

enum ResultTest {
    Ok(i32),
    Err(String),
    Unknown
}



fn divide_in_two(n: i32) -> ResultTest {
    if n % 2 == 0 {
        ResultTest::Ok(n/2)
    } else {
        ResultTest::Err(format!("cannot divide {n} into two equal parts"))
    }
}


pub fn pattern_matching_example_sixten() {
    let n = 100;
    match divide_in_two(n) {
        ResultTest::Ok(half) => println!("{n} divided in two is {half}"),
        ResultTest::Err(msg) => println!("sorry, an error happened: {msg}"),
        ResultTest::Unknown => println!("unknown error")
    }
}


// let control flow

use std::time::Duration;

fn sleep_for(secs: f32) {
    if let Ok(duration) = Duration::try_from_secs_f32(secs) {
        std::thread::sleep(duration);
        println!("slept for {duration:?}");
    } else {
         println!("sleep error");
    }
}


pub fn pattern_matching_example_seventen() {
    sleep_for(-10.0);
    sleep_for(0.8);
}



fn hex_or_die_trying(maybe_string: Option<String>) -> Result<u32, String> {
    if let Some(s) = maybe_string {
        if let Some(first_byte_char) = s.chars().next() {
            if let Some(digit) = first_byte_char.to_digit(16) {
                Ok(digit)
            } else {
                return Err(String::from("not a hex digit"));
            }
        } else {
            return Err(String::from("got empty string"));
        }
    } else {
        return Err(String::from("got None"));
    }
}

fn hex_or_die_trying_rewrite_version(maybe_string: Option<String>) -> Result<u32, String>{
    let Some(s) = maybe_string else {
        return Err(String::from("got None"));
    };
    let Some(first_byte_char) = s.chars().next() else {
        return Err(String::from("got empty string"));
    };
    let Some(digit) = first_byte_char.to_digit(16) else {
        return Err(String::from("not a hax digit"));
    };
    return Ok(digit)
}

pub fn pattern_matching_example_eighteen() {
    // println!("result: {:?}", hex_or_die_trying(Some(String::from("123"))));
    println!("result: {:?}", hex_or_die_trying_rewrite_version(Some(String::from("123"))));

}

pub fn pattern_matching_example_nineteen() {
    let mut name = String::from("Comprehensive Rust 🦀");
    while let Some(c) = name.pop(){
        println!("character: {c}");
    }
}