pub fn if_expression() {
    let x = 10;
    if x == 0 {
        println!("zero");
    } else if x < 100 {
        println!("biggish");
    } else {
        println!("huge");
    }
}

pub fn if_expression_with_return() -> &'static str {
    let x = 20;
    if x == 0 {
        "zero"
    } else if x < 100 {
        "biggish"
    } else {
        "huge"
    }
}

pub fn if_expression_with_one_line() {
    let x = 10;
    let size = if x < 20 {"small"} else {"large"};
    println!("Size is: {size}");
}