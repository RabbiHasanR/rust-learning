mod structs;
mod enums;
mod type_aliases;
mod consts;
mod static_;
mod elevator_event;

fn main() {
    structs::struct_example();
    structs::tuple_struct_example();
    structs::tuple_struct_with_units_example();
    enums::enum_example();
    enums::enum_repr();
    enums::enum_bits();

    type_aliases::type_alias_example();

    consts::const_example();
    static_::static_example();
    elevator_event::elevator_event_example();
    println!("All examples executed successfully.");
}
