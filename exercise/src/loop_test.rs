pub fn while_loop_test() {
    let mut x = 200;
    while x >= 10 {
        x = x / 2;
        println!("x is after division: {x}");
    }
    println!("final x: {x}");
}


pub fn for_loop_range_test() {
    for x in 1..=5 {
        println!("x: {x}");
    }

    for y in 6..10 {
        println!("y: {y}");
    }
}


pub fn for_loop_in_test() {
    for elem in [1,2,3,4,5] {
        println!("elem: {elem}");
    }
}



pub fn loop_test() {
    let mut i = 0;
    loop {
        i += 1;
        println!("loop i: {i}");
        if i > 100 {
            break;
        }
    }
}