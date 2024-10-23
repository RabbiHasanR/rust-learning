static BANNER: &str = "Welcome to RustOS 3.14";

pub fn static_example() {
    println!("{BANNER}");
}

// mutable static example..mutable static is not thread safe

static mut COUNTER: i32 = 0;

fn increment_counter() {
    unsafe {
        COUNTER += 1;
    }
}

pub fn static_example_two() {
    unsafe {
        println!("Initial COUNTER: {}", COUNTER);
    }

    increment_counter();

    unsafe {
        println!("Updated COUNTER: {}", COUNTER);
    }
}

// mutable static with thread safe way using Mutex

use std::sync::Mutex;

static COUNTER_TWO: Mutex<i32> = Mutex::new(0);

fn increment_counter_two() {
    let mut num = COUNTER_TWO.lock().unwrap();
    *num += 1;
}

pub fn static_example_three() {
    increment_counter_two();
    println!("COUNTER: {:?}", COUNTER_TWO.lock().unwrap());
}

// static with complex types. You can also use static to store more complex types, such as arrays or vectors,
// but remember that the value must be computable at compile time (for example, you cannot use dynamically allocated data).

static NUMBERS: [i32; 3] = [1, 2, 3];

pub fn static_example_four () {
    for number in NUMBERS.iter() {
        println!("{}", number);
    }
}