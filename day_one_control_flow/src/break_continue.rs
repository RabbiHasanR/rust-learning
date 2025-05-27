pub fn break_continue() {
    let mut i = 0;
    loop {
        i += 1;
        if i > 5 {
            break;
        }
        if i % 2 == 0 {
            continue;
        }

        dbg!(i);
    }
}

pub fn break_return() -> i32 {
    let mut i = 0;
    loop {
        i += 1;
        if i > 5 {
            break i * 2; // Return value when breaking
        }
        if i % 2 == 0 {
            continue; // Skip even numbers
        }
        dbg!(i); // Print odd numbers
    }
}