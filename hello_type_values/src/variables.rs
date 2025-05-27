pub fn variables() {
    let x: i32 = 10;
    println!("x: {x}");
    // x = 20;
    // println!("x: {x}"); // This line will cause a compile-time error because x is immutable
}