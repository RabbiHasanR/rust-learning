pub fn array_example() {
    let mut a: [i8; 10] = [42; 10];
    a[5] = 0;
    println!("a: {a:?}");
    println!("pretty a: {a:#?}");
    // println!("out of range: {}", a[11]); // out of bound error

    for element in a {
        println!("element: {element}");
    }

}


pub fn array_example_two() {
    let numbers: [i32;5] = [1,2,3,4,5];

    println!("Array: {:?}", numbers);
    println!("First element: {}", numbers[0]);
    println!("Array length: {}", numbers.len());
}


pub fn array_example_three() {
    let zeros: [i32; 5] = [0; 5];

    println!("Array of zeros: {:?}", zeros);
}

// mutants arrays
pub fn array_example_four() {
    let mut numbers: [i32; 4] = [1,2,3,4];
    numbers[0] = 10;
    println!("Update array: {:?}", numbers);
}


// accessing array elements safely

pub fn array_example_five() {
    let numbers = [1,2,3,4,5];

    match numbers.get(5) {
        Some(value) => println!("The value is: {}", value),
        None => println!("Index out of bounds!"),
    }
}


// iterating over an array

pub fn array_example_six() {
    let numbers = [10,20,30,40];

    for number in numbers.iter() {
        println!("The value is: {}", number);
    }
}


// multi dimensional array

pub fn array_example_seven() {
    let matrix: [[i32; 3]; 2] = [
        [1,2,3],
        [4,5,6],
    ];

    println!("Matrix: {:?}", matrix);

    //accessing elements
    println!("Element at [1][2]: {}", matrix[1][2]);
}


// array iteration

pub fn array_example_eight() {
    let primes = [2,3,5,7,11,13,17,19];

    for prime in primes {
        for i in 2..prime {
            assert_ne!(prime % i, 0);
        }
    }
}