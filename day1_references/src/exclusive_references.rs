pub fn exclusive_reference() {
    let mut point = (1, 2);
    let x_coord = &mut point.0; // Mutable reference to the first element
    let y_coord = &mut point.1; // Mutable reference to the second element
    // println!("before change mutable reference: ({point:?})");
    *x_coord = 20; // Change the first element through the mutable reference
    *y_coord = 30; // Change the second element through the mutable reference
    println!("after change mutable reference: ({point:?})")
}