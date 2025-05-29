struct Person {
    name: String,
    age: u8,
}

fn describe(person: &Person) {
    println!("{} is {} years old.", person.name, person.age);
}

pub fn struct_example() {
    let mut peter = Person {
        name: String::from("Peter"),
        age: 27
    };
    
    describe(&peter);
    peter.age = 28; // Modify the age
    describe(&peter);

    let name = String::from("Alice");
    let age = 39;
    let avery = Person { name, age }; // Create a new Person instance using destructuring
    describe(&avery);

    let jackie = Person { name: String::from("Jackie"), ..avery }; // Create a new Person instance using struct update syntax
    describe(&jackie);
}


// tuple structs

struct Point(i32, i32);

pub fn tuple_struct_example() {
    let p = Point(17, 23);
    println!("({}, {})", p.0, p.1); // Accessing tuple struct fields using index;
}

struct PoundsOfForce(f64);
struct Newtons(f64);

fn compute_thruster_force() -> PoundsOfForce {
    let force = 9.81;
    PoundsOfForce(force)
}

fn set_thruster_force(force: Newtons) {
    println!("Setting thruster force to {} N", force.0);
}

pub fn tuple_struct_with_units_example() {
    let force = compute_thruster_force();
    set_thruster_force(Newtons(force.0 * 0.224809)); // Convert PoundsOfForce to Newtons
}