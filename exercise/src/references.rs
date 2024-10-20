pub fn shared_reference() {
    let a = 'A';
    let b = 'B';
    let mut r: &char = &a;
    let mut k = r;
    println!("r: {}", *r);
    println!("r: {}", r);
    println!("k: {}", k);
    println!("r memory: {:p}", &r);
    println!("k memory: {:p}", &k);
    println!("a memory: {:p}", &a);
    r = &b;
    println!("r: {}", *r);
    println!("r: {}", r);
}


// dangling reference forbidden example

pub fn dangling_reference() {
    let r = 5;
    {
        let x = 5;
        // r = &x;
    }

    println!("r: {}", r);
}


pub fn reference_example() {
    let x = 10;
    println!("result: {:?}", x_axis(&x))
}

fn x_axis(x: &i32) -> (i32, i32) {
    let point = (*x, 0);
    return point;
}


// exclusive references

pub fn exclusive_reference() {
    let mut point = (1,2);
    let x_coord = &mut point.0;
    *x_coord = 20;
    let y_coord = &mut point.0;
    // *y_coord = 30;
    println!("y_coord: {y_coord:?}");
    // println!("x_coord: {x_coord:?}");
    println!("point: {point:?}");

    let mut x = 'A';
    let y = &mut x;
    *y = 'B';
    println!("y: {y}");
    println!("x: {x}");

    let mut p = "Fucking Rust";
    let q = &mut p;
    *q = "Fuck Fuck";
    // println!("p: {p}");
    println!("q: {q}");
}