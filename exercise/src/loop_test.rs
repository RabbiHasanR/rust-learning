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



pub fn loop_with_break_and_continue() {
    let mut i =0;
    let result = loop {
        i += 1;
        if i > 5 {
            break i;
        }
        if i % 2 == 0 {
            continue;
        }
        println!("break and continue: {}",i);
    };

    println!("return value when breaK in loop: {}", result);
}


pub fn while_loop_return() {
    let mut counter = 0;

    let result = while counter < 10 {
        counter += 1;
        println!("while counter: {counter}");
        if counter == 5 {
            return;
        }
    };

    println!("while loop return unit type: {:?}", result);

}


pub fn for_loop_return() {
    let result = for i in 1..5 {
        println!("for i: {i}");
        if i == 3 {
            return;
        }
    };

    println!("for loop return unit type: {:?}", result);
}
