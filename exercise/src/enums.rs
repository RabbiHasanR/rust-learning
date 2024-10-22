use std::mem::transmute;
use std::mem;


#[derive(Debug)]
enum Direction {
    Left,
    Right
}

#[derive(Debug)]
enum PlayerMove {
    Pass,
    Run(Direction),
    Teleport { x:u32, y:u32 },
}

pub fn enums_example() {
    let m: PlayerMove = PlayerMove::Run(Direction::Left);
    println!("On this turn: {:?}", m);
}

#[repr(u32)]
enum Bar {
    A,
    B = 10000,
    C,
}

pub fn enums_example_two() {
    println!("A: {}", Bar::A as u32);
    println!("B: {}", Bar::B as u32);
    println!("C: {}", Bar::C as u32);
}



macro_rules! dbg_bits {
    ($e:expr, $bit_type:ty) => {
        println!("- {}: {:#x}", stringify!($e), transmute::<_, $bit_type>($e))
    };
}


pub fn enums_example_three() {
    unsafe {
        println!("bool:");
        dbg_bits!(false, u8);
        dbg_bits!(true, u8);

        println!("Option<bool>:");
        dbg_bits!(None::<bool>, u8);
        dbg_bits!(Some(false), u8);
        dbg_bits!(Some(true), u8);

        println!("Option<Option<bool>>:");
        dbg_bits!(Some(Some(false)), u8);
        dbg_bits!(Some(Some(true)), u8);
        dbg_bits!(Some(None::<bool>), u8);
        dbg_bits!(None::<Option<bool>>, u8);


        println!("Option<&i32>:");
        dbg_bits!(None::<&i32>, usize);
        dbg_bits!(Some(&0i32), usize);

    }
}



enum DirectionTwo {
    North,
    South,
    East,
    West,
}


pub fn enums_example_four () {
    let dir = DirectionTwo::South;
    match dir {
        DirectionTwo::North => println!("Heading North!"),
        DirectionTwo::South => println!("Heading South!"),
        DirectionTwo::East => println!("Heading East!"),
        DirectionTwo::West => println!("Heading West!"),
    }
}


enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

pub fn enums_example_five () {
    println!("memory usage enums: {}", mem::size_of::<Message>());
    let msg1 = Message::Quit;
    let msg2 = Message::Move { x: 10, y: 20 };
    let msg3 = Message::Write(String::from("Hello, Enums!"));
    let msg4 = Message::ChangeColor(255, 0, 0);
    println!("memory usage enums two: {}", mem::size_of::<Message>());
    match msg2 {
        Message::Quit => println!("Quit!"),
        Message::Move { x, y } => println!("Move to ({}, {})", x, y),
        Message::Write(text) => println!("Message: {}", text),
        Message::ChangeColor(r, g, b) => println!("Change color of RGB({}, {}, {})", r, g, b),
    }
}



enum Light {
    On,
    Off,
}

impl Light {
    fn status(&self) {
        match self {
            Light::On => println!("The light is on."),
            Light::Off => println!("The light is off."),
        }
    }
}

pub fn enums_example_six() {
    let light = Light::On;
    light.status();
}

// enums and pattern matching

enum Shape {
    Circle(f64),
    Rectangle(f64, f64),
}

pub fn enums_example_seven() {
    let shape = Shape::Rectangle(7.8, 3.0);

    match shape {
        Shape::Circle(radius) => println!("It's a circle with radius {}", radius),
        Shape::Rectangle(width, height ) => println!("It's a rectangle with width {} and height {}", width, height),
    }
}


// enum Option<T> {
//     Some(T),
//     None,
// }

pub fn enums_example_eight() {
    let some_number = Some(8);
    let no_number: Option<i32> = None;

    match some_number {
        Some(value) => println!("The number is: {}", value),
        None => println!("No number found"),
    }

    match no_number {
        Some(value) => println!("The number is: {}", value),
        None => println!("No number found"),
    }

}



// Enums and Result types

fn divide(a:i32, b:i32) -> Result<i32, String> {
    if b == 0 {
        Err(String::from("Division by zero error"))
    } else {
        Ok(a / b)
    }
}


pub fn enums_example_nine() {
    let result = divide(50, 10);
    match result {
        Ok(value) => println!("Result: {}", value),
        Err(e) => println!("Error: {}", e),
    }

    let error_result = divide(4, 0);

    match error_result {
        Ok(value) => println!("Result: {}", value),
        Err(e) => println!("Error: {}", e),
    }
}