fn factorial(n: u32) -> u32 {
    let mut product = 1;
    for i in 1..=n {
        product *= dbg!(i);
    }
    product
}

fn fizzbuzz(n: u32) -> u32 {
    todo!()
}

pub fn macros_example() {
    let n = 4;
    println!("{n}! = {}", factorial(n));
    // fizzbuzz(n);
}



// declarative macros (macro_rules)

#[macro_export]
macro_rules! say_hello {
    () => {
        println!("Hello, World!");
    };
}

// macro with arguments

#[macro_export]
macro_rules! print_sum {
    ($x:expr, $y:expr) => {
        println!("The sum of {} and {} is {}", $x, $y, $x + $y);
    };
}


// repeating patterns in macros
#[macro_export]
macro_rules! sum {
    ($($num:expr),*) => {
        {
            let mut sum = 0;
            $(
                sum += $num;
            )*
            sum
        }
    };
}