pub fn strings() {
    let s1: &str = "World";
    println!("s1: {s1}");

    let mut s2: String = String::from("Hello ");
    println!("s2: {s2}");

    s2.push_str(s1);
    println!("s2: {s2}");

    let s3: &str = &s2[2..9]; // Create a slice of s2
    println!("s3: {s3}");
}


pub fn string_one() {
    println!("{:?}", b"abc");
    println!("{:?}", &[97, 98, 99]);
}

pub fn string_two() {
    println!(r#"<a href="link.html">link</a>"#);
    println!("<a href=\"link.html\">link</a>");
}