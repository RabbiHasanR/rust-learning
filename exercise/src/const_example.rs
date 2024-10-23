const DIGEST_SIZE: usize = 3;
const ZERO: Option<u8> = Some(42);

fn compute_digest(text: &str) -> [u8; DIGEST_SIZE] {
    let mut digest = [ZERO.unwrap_or(0); DIGEST_SIZE];

    for (idx, &b) in text.as_bytes().iter().enumerate() {
        digest[idx % DIGEST_SIZE] = digest[idx % DIGEST_SIZE].wrapping_add(b);
    }
    digest
}


pub fn const_example() {
    let digest = compute_digest("Hello");
    println!("digest: {digest:?}");
}



// const function example

const fn square(x: i32) -> i32 {
    x * x
}

const SQUARE_OF_FIVE: i32 = square(5);

pub fn const_example_two() {
    println!("Square of 5 is: {SQUARE_OF_FIVE}");
}


// example of arithmetic consts

const SECONDS_IN_MINUTES: u32 = 60;
const  SECONDS_IN_HOUR: u32 = SECONDS_IN_MINUTES * 60;
const SECONDS_IN_DAY: u32 = SECONDS_IN_HOUR * 24;

pub fn const_example_three() {
    println!("Seconds in a day: {}", SECONDS_IN_DAY);
}