#[derive(Debug, Clone, Default)]
struct Player {
    name: String,
    strength: u8,
    hit_points: u8,
}

pub fn derive_example() {
    let p1 = Player::default();
    let mut p2 = p1.clone();
    p2.name = String::from("EldurScrollz");
    println!("{p1:?} vs. {p2:?}");
}


#[derive(Debug, Clone, PartialEq, Default)]
struct Point {
    x: i32,
    y: i32,
}

pub fn derive_example_two() {
    let point1 = Point {x:10, y:20};
    let point2 = point1.clone(); // Uses Clone trait

    println!("{:?}", point1); // Uses Debug trait
    println!("Points are equal: {}", point1 == point2); // Uses PartialEq trait

    let default_point = Point::default(); // Uses Default trait
    println!("Default point: {:?}", default_point);
}