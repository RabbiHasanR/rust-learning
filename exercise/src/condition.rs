pub fn conditon_practice() {
    let x = 10;
    if x == 0 {
        println!("Zero!");
    } else if x < 100 {
        println!("biggish");
    } else {
        println!("huge");
    }
}

pub fn conditon_as_expression() {
    let x = 10;
    let size = if x < 20 { "small" } else { "large" };
    println!("number size: {}", size);
}
