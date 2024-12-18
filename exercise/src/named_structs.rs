struct Person {
    name: String,
    age: u8,
}

fn describe(person: &Person) {
    println!("{} is years {} old", person.name, person.age);
}

pub fn named_struct_example() {
    let mut peter = Person{ name: String::from("Peter"), age: 27 };
    describe(&peter);
    peter.age = 28;
    peter.name = String::from("Rabbi");
    describe(&peter);

    let name = String::from("Avery");
    let age = 39;
    let avery = Person{ name, age};
    describe(&avery);

    let jackie = Person{name: String::from("Jackie"), ..avery};
    describe(&jackie);
}