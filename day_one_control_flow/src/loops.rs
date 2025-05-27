pub fn while_loop() {
    let mut x = 200;
    while x >= 10 {
        x = x / 2;
    }

    dbg!(x);
}


pub fn for_loop() {
    for x in 1..5 {
        dbg!("Looping through range: {}", x);
    }

    for elem in [2, 4, 8, 16, 32] {
        dbg!("Looping through array: {}", elem);
    }
}

pub fn loop_statement() {
    let mut i = 0;
    loop {
        i += 1;
        dbg!(i);
        if i > 100 {
            break;
        }
    }
}