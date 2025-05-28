pub fn shared_reference() {
    let a = 'A';
    let b = 'B';

    let mut r: &char = &a;
    dbg!(r);
    r = &b; // Reassigning the reference to point to a different value
    dbg!(r);

    // this line would cause a compile-time error if uncommented, as it tries to modify a value through an immutable reference
    // *r = 'C';
    // dbg!(r); // This will not change the original value of 'b', as `r` is a reference to 'b'
}