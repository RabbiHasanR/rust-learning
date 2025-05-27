fn gcd(a: u32, b: u32) -> u32 {
    if b > 0 { gcd(b, a % b) } else { a }
}

pub fn functions() {
    let a = 143;
    let b = 52;
    dbg!(gcd(a, b));
}