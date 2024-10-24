mod blocks_scopes;
mod condition;
mod functions;
mod labels;
mod loop_test;
mod macros;
mod array;
mod tuple;
mod nested_arrays;
mod collatz_sequence;
mod references;
mod geometry;
mod named_structs;
mod tuple_structs;
mod enums;
mod const_example;
mod static_example;
mod type_alias;
mod elevator_events;
mod pattern_matching;

use my_proc_macro::HelloMacro;
// use my_proc_macro::route;
use my_proc_macro::my_macro;

// Define the HelloMacro trait
pub trait HelloMacro {
    fn hello();
}

#[derive(HelloMacro)]
struct Pancakes;



// #[route("/home")]
// fn home() {
//     // Function body
// }
fn main() {
    // println!("Hello 🌍!");
    // println!("Edit me!");

    // let mut x: i32 = 10;
    // println!("x: {x}");

    // let x = 20;
    // x = 20;
    // println!("x: {x}");

    // let s1 = String::from("hello");
    // let s2 = s1;
    // println!("{}", s2);

    // let x = 5;
    // let y = x;
    // println!("x: {}, y: {}", x, y);

    // immutable reference
    // let x = 5;
    // let y = &x;
    // println!("x: {}, y: {}", x, y);

    // mutable reference
    // let mut x = 5;
    // let y = &mut x;
    // *y += 1;
    // println!("x: {}", x);

    // constant variables
    // const MAX_POINTS: u32 = 100_000;
    // println!("{}", MAX_POINTS);

    //arithmetic operation
    // println!("result: {}", interproduct(120, 100, 248));

    // let a = 10;
    // let b = 20;
    // takes_u32(a);
    // takes_i8(b);
    // takes_u32(b); error

    // let n = 20;
    // println!("fb({n}) = {}", fib(n));

    // condition

    // condition::conditon_practice();
    // condition::conditon_as_expression();

    // loop
    // loop_test::while_loop_test();
    // loop_test::for_loop_range_test();admin_view
    // loop_test::for_loop_in_test();
    // loop_test::loop_test();

    // break and continue
    // loop_test::loop_with_break_and_continue();
    // loop_test::while_loop_return();
    // loop_test::for_loop_return();

    // labels
    // labels::labels_example();
    // labels::labels_example_two();
    // labels::labels_example_three();
    // labels::labels_example_four();

    // labels::arbitray_block_example();
    // let x = labels::arbitray_block_example_find_value(-5);
    // let y = labels::arbitray_block_example_find_value(0);
    // let z = labels::arbitray_block_example_find_value(10);

    // println!("Value: {x}");
    // println!("Value: {y}");
    // println!("Value: {z}");

    // block scopes
    // blocks_scopes::block_example();
    // let x = blocks_scopes::block_example_return();
    // println!("return x: {x}");

    // blocks_scopes::scope_example();
    // blocks_scopes::scope_example_two();

    // let y = 10;
    // blocks_scopes::function_scope(y);
    // blocks_scopes::ownership_borrowing_scope();
    // blocks_scopes::return_value_scope();

    // functions
    // let gcd_result = functions::gcd(143, 52);
    // println!("function gcd result: {gcd_result}");

    // functions::pass_args_as_reference();
    // functions::mutable_ref();
    // functions::return_multiple_value();

    // functions::generic_example();
    // functions::passing_func_as_arguments();
    // functions::return_function_example();
    // functions::closures_example();
    // functions::recursion_example();

    // macros

    // macros::macros_example();
    // say_hello!();
    // print_sum!(5, 10);
    // let result = sum!(1, 2, 3, 4, 5);
    // println!("The sum is: {}", result);

    // Pancakes::hello();

    // // home();
    // my_macro!(Hello, Rust!);

    // let result = collatz_sequence::collatz_length(11);
    // let result2 = collatz_sequence::collatz_length(10);

    // println!("Length: {result}");
    // println!("Length2: {result2}");

    // array::array_example();
    // array::array_example_two();
    // array::array_example_three();
    // array::array_example_four();
    // array::array_example_five();
    // array::array_example_six();
    // array::array_example_seven();
    // array::array_example_eight();

    // tuple::tuple_example();
    // tuple::tuple_example_two();
    // tuple::tuple_example_three();
    // tuple::tuple_example_four();
    // tuple::tuple_example_five();
    // tuple::tuple_example_six();

    // nested_arrays::nested_arrays_exercise();

    // references::shared_reference();
    // references::dangling_reference();
    // references::reference_example();
    // references::exclusive_reference();

    // references::slices_example();
    // references::slices_example_two();

    // references::string_example();

    // references::string_slice_example();

    // geometry::geometry();

    // named_structs::named_struct_example();

    // tuple_structs::tuple_structs_example();

    // enums::enums_example();
    // enums::enums_example_two();
    // enums::enums_example_three();
    // enums::enums_example_four();
    // enums::enums_example_five();
    // enums::enums_example_six();
    // enums::enums_example_seven();
    // enums::enums_example_eight();
    // enums::enums_example_nine();

    // const_example::const_example();
    // const_example::const_example_two();
    // const_example::const_example_three();

    // static_example::static_example();
    // static_example::static_example_two();
    // static_example::static_example_three();
    // static_example::static_example_four();

    // type_alias::type_alias_example();
    // type_alias::type_alias_example_two();
    // type_alias::type_alias_example_three();
    // type_alias::type_alias_example_four();
    // type_alias::type_alias_example_five();

    // elevator_events::elevator_event();

    pattern_matching::pattern_matching_example();
    pattern_matching::pattern_matching_example_two();
    pattern_matching::pattern_matching_example_three();
    pattern_matching::pattern_matching_example_four();
    pattern_matching::pattern_matching_example_five();
    pattern_matching::pattern_matching_example_six();
    pattern_matching::pattern_matching_example_seven();
    pattern_matching::pattern_matching_example_eight();
    pattern_matching::pattern_matching_example_nine();
    pattern_matching::pattern_matching_example_ten();
    pattern_matching::pattern_matching_example_eleven();
    pattern_matching::pattern_matching_example_twelve();
    pattern_matching::pattern_matching_example_thirteen();
    pattern_matching::pattern_matching_example_fourteen();
}

// fn interproduct(a: i32, b: i32, c: i32) -> i32 {
//     return a * b + b * c + c * a;
// }

// fn takes_u32(x: u32) {
//     println!("u32: {x}");
// }

// fn takes_i8(y: i8) {
//     println!("i8: {y}");
// }

// fn fib(n: u32) -> u32 {
//     if n < 2 {
//         return n;
//     } else {
//         return fib(n - 1) + fib(n - 2);
//     }
// }
