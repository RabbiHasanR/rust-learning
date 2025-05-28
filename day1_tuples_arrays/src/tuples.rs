pub fn tuples() {
    let t: (i8, bool) = (7, true);
    dbg!(t.0); // Accessing the first element of the tuple
    dbg!(t.1); // Accessing the second element of the tuple

    // tuples are not IntoIterator, so we cannot use a for loop directly
    // for elem in t {
    //     dbg!(elem); // Print each element of the tuple
    // }
}


fn check_order(tuple: (i32, i32, i32)) -> bool {
    let (left, middle, right) = tuple; // Destructuring the tuple
    left < middle && middle < right // Check if the elements are in ascending order
}

pub fn check_order_example() {
    let tuple = (1, 5, 3);
    println!("{tuple:?}: {}", if check_order(tuple) { "Ordered" } else {" Unordered "});
}