pub fn block_scope() {
    let z = 13;
    let x = {
        let y = 5;
        dbg!(y);
        z - y
    };
    dbg!(x);
    dbg!(z);
    // dbg!(y); // This line will cause a compile-time error because y is not in scope here
}