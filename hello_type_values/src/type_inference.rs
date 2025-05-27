fn takes_u32(x: u32) {
    println!("u32: {x}");
}

fn takes_i8(y: i8) {
    println!("i8: {y}");
}


pub fn type_inference() {
    let x = 10;
    let y = 20;

    takes_u32(x);
    takes_i8(y);
    // takes_u32(y); // This will cause a compile-time error because y is i8 and cannot be passed to takes_u32
    takes_u32(y as u32); // This will work because we explicitly cast y to u32
}