trait Animal {
    fn leg_count(&self) -> u32;
}


trait Pet: Animal {
    fn name(&self) -> String;
}

struct Dog(String);

impl Animal for Dog {
    fn leg_count(&self) -> u32 {
        4
    }
}

impl Pet for Dog {
    fn name(&self) -> String {
        self.0.clone()
    }
}


pub fn supertraits_example() {
    let puppy = Dog(String::from("Rex"));
    println!("{} has {} legs", puppy.name(), puppy.leg_count());
}

use std::fmt;

trait Displayable: fmt::Display {
    fn display_info(&self);
}

trait Printable: Displayable {
    fn print(&self) {
        println!("Printable: {}", self);
    }


}

struct Person {
    name: String,
}

impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Displayable for Person {
    fn display_info(&self) {
        println!("Name: {}", self.name);
    }
}

impl Printable for Person {}

pub fn supertraits_example_two() {
    let person = Person { name: "Alice".to_string() };
    person.display_info();
    person.print();
}


// building multiple level of traits

trait Readable {
    fn read(&self) -> String;
}

trait Writable {
    fn write(&self, content: &str);
}

trait Document: Readable + Writable {
    fn save(&self);
}


struct TextFile {
    content: String,
}

impl Readable for TextFile {
    fn read(&self) -> String {
        self.content.clone()
    }
}

impl Writable for TextFile {
    fn write(&self, content: &str) {
        println!("Writing content: {}", content);
    }
}

impl Document for TextFile {
    fn save(&self) {
        println!("Saving document: {}", self.content);
    }
}


pub fn supertraits_example_three() {
    let text_file = TextFile {
        content: "Hello, Rust!".to_string(),
    };

    println!("Read: {}", text_file.read());
    text_file.write("New content");
    text_file.save();
}

// calling supertrait methods explicitly

trait Speak {
    fn say(&self) {
        println!("Hello from Speak");
    }
}

trait Whisper: Speak {
    fn say(&self) {
        println!("Hello from Whisper");
    }
}

struct PersonTwo;

impl Speak for PersonTwo {}

impl Whisper for PersonTwo {}

pub fn supertraits_example_four() {
    let person = PersonTwo;

    // Call the overridden method from Whisper
    Whisper::say(&person); // Output: Hello from Whisper

    // Call the method from Speak explicitly
    Speak::say(&person); // Output: Hello from Speak
}