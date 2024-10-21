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

// slices
// immutable borrowing slice example
pub fn slices_example() {
    let a: [i32; 6] = [10,20,30,40,50,60];

    println!("a: {a:?}");
    let s = &a[2..4];
    // let x = a[2..4];
    println!("s: {s:?}");
}


// mutable borrowing slice example
pub fn slices_example_two() {
    let mut a = [1,2,3,4,5,6,7,8,9,10];
    println!("a: {a:?}");

    let mut x = &mut a[3..8];
    println!("x: {x:?}");
    println!("x slice first item: {}", x[0]);
    x[0] = 10;
    println!("x slice first item: {}", x[0]);
    println!("a: {a:?}");
}

// strings

pub fn string_example() {
    let s1 = "world";
    println!("s1:{s1}");

    let mut s2 = String::from("Hello ");
    println!("s2: {s2}");
    s2.push_str(s1);
    println!("s2: {s2}");

    let s3 = &s2[s2.len()- s1.len()..];
    println!("s3: {s3}");

    println!("{:?}", b"abc");
    println!("{:?}", &[97, 98, 99]);

    println!(r#"<a href="link.html">link</a>"#);
    println!("<a href=\"link.html\">link</a>");
}


// string slice using &str

pub fn string_slice_example() {
    let greeting: &str = "Hello, world";
    println!("{}", greeting);

    let part = &greeting[0..5];
    println!("Sliced: {}", part);

    let mut x = "fuck";
    println!("{x}");
    x = "hello";
    println!("{x}");
}