pub fn block_example() {
    let z = 13;
    let x = {
        let y = 10;
        println!("y: {y}");
        // z - y;
        z - y
    };

    println!("x: {x}");
    // println!("x: {:?}", x);
}


pub fn block_example_return() -> i32 {
    let x = {
        let y = 5;
        return y + 1;
    };
    println!("This will never print, because return exits the function");
}



pub fn scope_example() {
    let a = 10;
    println!("before: {a}");
    {
        let a = "hello";
        println!("inner scope: {a}");

        let a = true;
        println!("shadowed in inner scope: {a}");
    }
    println!("after: {a}");
}


pub fn scope_example_two() {
    let x = String::from("Hello Rabbi");
    {
        let y = 10;
        println!("Inside inner block: x = {x}, y={y}");
    }

    println!("outside innder block: x={x}");
}


pub fn function_scope(x: i32) {
    println!("x in my function: {x}");
}

pub fn ownership_borrowing_scope() {
    let s = String::from("ownership");
    take_ownership(s);
    // println!("string ownership: {s}"); // This would cause an error because `s` is no longer in scope
    let x = 5;
    makes_copy(x);
    println!("x is still accessible: {x}");
}

fn take_ownership(some_string: String) {
    println!("Took ownership of: {some_string}");
}

fn makes_copy(some_integer: i32) {
    println!("copied integer: {some_integer}");
}



pub fn return_value_scope() {
    let s1 = gives_ownership();
    let s4 = gives_ownership();
    println!("s1: {s1}");
    println!("s4: {s4}");

    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);
    // println!("s2: {s2}"); // This would cause an error because `s2` is no longer valid
    println!("s3: {s3}");

}

fn gives_ownership() -> String {
    let some_string = String::from("ownership return");
    return some_string;
}

fn takes_and_gives_back(a_string: String) -> String {
    return  a_string;
}