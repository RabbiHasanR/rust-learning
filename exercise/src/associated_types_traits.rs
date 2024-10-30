#[derive(Debug)]
struct Meters(i32);
#[derive(Debug)]
struct MetersSquared(i32);


trait Multiply {
    type Output;
    fn multiply(&self, other: &Self) -> Self::Output;
}

impl Multiply for Meters {
    type Output = MetersSquared;
    fn multiply(&self, other: &Self) -> Self::Output {
        MetersSquared(self.0 * other.0)
    }
}


pub fn trait_example() {
    println!("{:?}", Meters(10).multiply(&Meters(20)));
}



// associated types example in traits

trait Container {
    type Item;

    fn add_item(&mut self, item: Self::Item);
    fn get_item(&self) -> Option<&Self::Item>;
}


struct Bag {
    items: Vec<String>,
}

impl Container for Bag {
    type Item = String;
    fn add_item(&mut self, item: Self::Item) {
        self.items.push(item);
    }
    fn get_item(&self) -> Option<&Self::Item> {
        self.items.get(0)
    }
}



pub fn trait_example_two() {
    let mut my_bag = Bag { items: Vec::new() };

    my_bag.add_item("Rust Programming".to_string());
    my_bag.add_item("Ownership Concepts".to_string());

    if let Some(item) = my_bag.get_item() {
        println!("First item: {}", item);
    }
}


struct Counter {
    count: u32,
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}


pub fn trait_example_three() {
    let mut counter = Counter { count:0 };

    while let Some(value) = counter.next() {
        println!("Counter: {}", value);
    }
}