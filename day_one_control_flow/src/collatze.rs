pub fn collatz_length(mut n: u32) -> u32 {
    let mut length = 1;
    loop {
        if n == 1 {
            break length;
        } else if n % 2 == 0 {
            n /= 2;
        } else {
            n = 3 * n + 1;
        }
        length += 1;
    }
}