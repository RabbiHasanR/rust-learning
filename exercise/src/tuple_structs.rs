struct Point(i32, i32);

pub fn tuple_structs_example() {
    let p = Point(17,32);
    println!("({}, {})", p.0, p.1);
}

struct PoundsOfForce(f64);
struct Newtons(f64);

fn compute_thruster_force() -> PoundsOfForce {
    let pounds_of_force = PoundsOfForce(100.89);
    pounds_of_force
}

fn set_thruster_force(force: Newtons) {

}

pub fn tuple_example_two() {
    let force = compute_thruster_force();
    let newton = Newtons(force.0);
    set_thruster_force(newton);
}