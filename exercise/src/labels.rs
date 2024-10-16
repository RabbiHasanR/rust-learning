

pub fn labels_example() {
    'outer: loop {
        println!("In the outer loop");

        loop {
            println!("In the inner loop");
            break 'outer;
        }
        println!("This will never be printed");
    }
    println!("Exited the outer loop");
}



pub fn labels_example_two() {
    'outer: for i in 0..5 {
        for j in 0..5 {
            if i == 2 && j == 2 {
                continue 'outer;
            }
            println!("labels example i = {}, j = {}", i, j);
        }
    }
}


pub fn labels_example_three() {
    let mut found = false;
    'outer: for i in 1..5 {
        for j in 1..5 {
            if i * j == 6 {
                found = true;
                break 'outer;
            }
        }
    }

    if found {
        println!("Found a product that equals 6");
    }
}


pub fn labels_example_four() {
    let s = [[5,6,7], [8,9,10], [21,15,32]];
    let mut elements_searched = 0;
    let target_value = 10;
    'outer: for i in 0..s.len() {
        for j in 0..s[i].len() {
            elements_searched += 1;
            println!("value: {}", s[i][j]);
            if s[i][j] == target_value {
                break 'outer;
            }
        }
    }
    println!("elements searched : {elements_searched}");
}


pub fn arbitray_block_example() {
    let result = 'block: {
        let x = 10;
        if x == 10 {
            break 'block 42;
        }
        0
    };
    println!("The result is: {}", result);
}


pub fn arbitray_block_example_find_value(x: i32) -> i32 {
    'block: {
        if x < 0 {
            break 'block -1;
        }
        if x == 0 {
            break 'block 0;
        }
        x * 2
    }
}