pub fn slice() {
    let a: [i32; 6] = [10, 20, 30, 40, 50, 60];
    println!("a: {a:?}");

    let s: &[i32] = &a[2..4]; // Create a slice from the array
    println!("a: {a:?}");
    println!("s: {s:?}");

}