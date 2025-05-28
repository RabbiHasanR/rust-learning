pub fn reference_valid() {
    let x_ref = {
        let x = 10;
        // 12 + x // x is valid within this block
        // &x // x is a reference to a value that goes out of scope. this will cause a compile-time error

    };
    dbg!(x_ref); // This will cause a compile-time error because x goes out of scope
}