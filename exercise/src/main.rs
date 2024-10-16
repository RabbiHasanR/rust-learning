mod condition;
mod loop_test;


fn main() {
    println!("Hello 🌍!");
    println!("Edit me!");

    let mut x: i32 = 10;
    println!("x: {x}");

    // let x = 20;
    x = 20;
    println!("x: {x}");

    let s1 = String::from("hello");
    let s2 = s1;
    println!("{}", s2);

    let x = 5;
    let y = x;
    println!("x: {}, y: {}", x, y);

    // immutable reference
    let x = 5;
    let y = &x;
    println!("x: {}, y: {}", x, y);

    // mutable reference
    let mut x = 5;
    let y = &mut x;
    *y += 1;
    println!("x: {}", x);

    // constant variables
    const MAX_POINTS: u32 = 100_000;
    println!("{}", MAX_POINTS);

    //arithmetic operation
    println!("result: {}", interproduct(120, 100, 248));


    let a = 10;
    let b = 20;
    takes_u32(a);
    takes_i8(b);
    // takes_u32(b); error

    let n = 20;
    println!("fb({n}) = {}", fib(n));


    // condition 

    condition::conditon_practice();
    condition::conditon_as_expression();

    // loop
    loop_test::while_loop_test();
    loop_test::for_loop_range_test();
    loop_test::for_loop_in_test();
    loop_test::loop_test();

    // break and continue
    loop_test::loop_with_break_and_continue();
    loop_test::while_loop_return();
    loop_test::for_loop_return();
}



fn interproduct(a: i32, b: i32, c: i32) -> i32 {
    return a * b + b * c + c * a;
}


fn takes_u32(x: u32) {
    println!("u32: {x}");
}

fn takes_i8(y: i8) {
    println!("i8: {y}");
}


fn fib(n: u32) -> u32 {
    if n < 2 {
        return n;
    } else {
        return fib(n - 1) + fib(n - 2);
    }
}