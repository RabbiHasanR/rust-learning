#[derive(Debug)]
enum CarrayableConcreteItem {
    Left,
    Right
}

type Item = CarrayableConcreteItem;

pub fn type_alias_example() {
    let item: Item = Item::Left;
    println!("Item: {:?}", item);
}

// Aliases are more useful with long, complex types
use std::cell::RefCell;
use std::sync::{Arc, RwLock};

type PlayerInventory = RwLock<Vec<Arc<RefCell<Item>>>>;