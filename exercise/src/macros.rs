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
