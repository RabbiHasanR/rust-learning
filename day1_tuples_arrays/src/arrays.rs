pub fn arrays () {
    let mut a: [i8; 5] = [5, 4, 3, 2, 1];
    a[2] = 0;
    println!("Array a: {:?}", a); // Print the array in debug format

    println!("Array a: {:#?}", a); // Pretty print the array

    for elem in a {
        println!("Element: {}", elem); // Print each element
    }
}


pub fn array_iteraton() {
    // let primes: [i32; 8] = [2, 3, 5, 7, 11, 13, 17, 19];
    let primes = [2, 3, 5, 7, 11, 13, 17, 19, 20];

    for prime in primes {
        for i in 2..prime {
            assert_ne!(prime % i, 0, "Found a non-prime number: {}", prime); // assert_ne checks that the condition is not equal to the second argument
        }
    }
}