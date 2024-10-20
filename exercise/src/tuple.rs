pub fn tuple_example() {
    let t: (i8, bool) = (7, true);
    println!("t.0: {}", t.0);
    println!("t.1: {}", t.1);
}

pub fn tuple_example_two() {
    let person: (&str, i32, f64) = ("Alice", 30, 65.5);

    println!("Name: {}", person.0);
    println!("Age: {}", person.1);
    println!("Weight: {}", person.2);
}

pub fn tuple_example_three() {
    let my_tuple = ("Rust", 2023, 3.14, true);

    println!("Language: {}", my_tuple.0);
    println!("Year: {}", my_tuple.1);
    println!("Version: {}", my_tuple.2);
    println!("Is Stable: {}", my_tuple.3);
}

// destructuring tuples
pub fn tuple_example_four() {
    let my_tuple = ("Rust", 2023, 3.14);
    let (lang, year, version) = my_tuple;
    println!("Language: {}", lang);
    println!("Year: {}", year);
    println!("Version: {}", version);
}

// return tuple

fn calculate_area_and_perimeter(length: f64, width: f64) -> (f64, f64) {
    let area = length * width;
    let perimeter = 2.0 * (length + width);
    (area, perimeter)
}

pub fn tuple_example_five() {
    let (area, perimeter) = calculate_area_and_perimeter(5.0, 3.0);
    println!("Area: {}", area);
    println!("Perimeter: {}", perimeter);
}

// nested tuple

pub fn tuple_example_six() {
    let nested_tuple = ((1, 2, 3), "Hello", (true, 4.5));

    println!("First element: {:?}", nested_tuple.0);
    println!("Second element: {}", nested_tuple.1);
    println!("Third element: {:?}", nested_tuple.2);

    println!("First element of the first tuple: {}", (nested_tuple.0).0);

    let unit: () = ();
    println!("Unit value: {:?}", unit);
}
