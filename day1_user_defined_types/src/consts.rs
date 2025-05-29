const DIGEST_SIZE: usize = 3;
const FILL_VALUE: u8 = calculate_fill_value();

const fn calculate_fill_value() -> u8 {
    if DIGEST_SIZE < 10 { 42 } else { 13 }
}

fn compute_digest(text: &str) -> [u8; DIGEST_SIZE] {
    // println!("fill value: {}", FILL_VALUE);
    // println!("digest size: {}", DIGEST_SIZE);
    let mut digest = [FILL_VALUE; DIGEST_SIZE];
    for (idx, &b) in text.as_bytes().iter().enumerate() {
        // println!("Processing byte: {}", b);
        // println!("Index: {}", idx);
        // println!("reminder: {}", idx % DIGEST_SIZE);
        // println!("Current digest value: {}", digest[idx % DIGEST_SIZE]);
        digest[idx % DIGEST_SIZE] = digest[idx % DIGEST_SIZE].wrapping_add(b);
    }
    digest
}

pub fn const_example() {
    let digest = compute_digest("Hello, world!");
    println!("Digest: {:?}", digest);
}