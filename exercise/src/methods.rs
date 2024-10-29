#[derive(Debug)]
struct Race {
    name: String,
    laps: Vec<i32>,
}


impl Race {
    fn new(name: &str) -> Self {
        Self {name: String::from(name), laps: Vec::new() }
    }

    fn add_lap(&mut self, lap: i32) {
        self.laps.push(lap);
    }

    fn print_laps(&self) {
        println!("Recorded {} laps for {}:", self.laps.len(), self.name);
        for (idx, lap) in self.laps.iter().enumerate() {
            println!("Lap {idx}: {lap} sec");
        }
    }

    fn finish(self) {
        let total: i32 = self.laps.iter().sum();
        println!("Race {} is finished, total lap time: {}", self.name, total);
    }
}


pub fn method_example() {
    let mut race = Race::new("Monaco Grand Prix");
    race.add_lap(70);
    race.add_lap(68);
    race.print_laps();
    race.add_lap(71);
    race.print_laps();
    race.finish();
}


#[derive(Debug)]
struct Book {
    title: String,
    pages: u32,
}

// struct reciver explain with example

impl Book {
    // &self: Shared, Immutable Reference
    // Using &self borrows the struct immutably. This means the method can read the struct’s data but cannot modify it.
    // After calling the method, the caller still owns the struct, and the struct is left unchanged.
    fn display_title(&self) {
        println!("Title: {}", self.title);
    }

    // &mut self: Unique, Mutable Reference
    // Using &mut self borrows the struct mutably. This allows the method to modify the struct’s data.
    // After calling the method, the caller still owns the struct, and any modifications are retained.
    fn add_pages(&mut self, new_pages: u32) {
        self.pages += new_pages;
    }

    // self: Takes Ownership
    // Using self takes ownership of the struct, meaning the struct is moved into the method.
    // The caller can no longer use the struct after calling this method, unless the method explicitly returns ownership of the struct.
    fn consume(self) {
        println!("Consuming book: {}", self.title);
    }

    // mut self: Takes Ownership and Allows Mutation
    // mut self takes ownership of the struct, like self, but allows the method to modify the struct.
    // After calling this method, the struct cannot be used unless it’s returned by the method.
    fn update_title(mut self, new_title: String) -> Self {
        self.title = new_title;
        self // Return ownership of self so it can still be used by the caller
    }

    // No Receiver: Static Method
    // A method without any receiver (self, &self, or &mut self) is a static method. This method does not operate on an instance of the struct.
    // Typically, static methods are used for constructors or utility functions.
    fn new(title: &str, pages: u32) -> Self {
        Self {
            title: title.to_string(),
            pages
        }
    }

}

pub fn method_example_two() {
    let my_book = Book {
        title: String::from("Rust Programming"),
        pages: 300,
    };
    my_book.display_title();
    println!("{:?}", my_book);  // my_book can still be used here when use &self recevier in method
}


pub fn method_example_three() {
    let mut my_book = Book {
        title: String::from("Rust Programming"),
        pages: 300,
    };
    
    my_book.add_pages(50);
    println!("{:?}", my_book); // my_book can still be used here, with updated pages when use &mut self recevier in method
}


pub fn method_example_four() {
    let my_book = Book {
        title: String::from("Rust Programming"),
        pages: 300,
    };
    my_book.consume();
    // println!("{:?}", my_book); // This would cause a compile error, as my_book is moved when use self recevier in method
}


pub fn method_example_five() {
    let my_book = Book {
        title: String::from("Rust Programming"),
        pages: 300,
    };

    let my_book = my_book.update_title(String::from("Advanced Rust"));
    println!("{:?}", my_book); // The updated book can still be used, because ownership is returned when use mut self recevier in method and return
}

pub fn method_example_six() {
    let my_book = Book::new("The Rust Programming", 300);
    println!("{:?}", my_book);
}