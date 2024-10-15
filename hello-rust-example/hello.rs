fn main() {
    println!("Hello World!");
    println!("I'm a Rustacean!");

    // comment in rust
    let x = 5 + /* 90 + */ 5;
    println!("Is `x` 10 or 100? x = {}", x);

    // formatted print

    // In general, the `{}` will be automatically replaced with any
    // arguments. These will be stringified.
    println!("{} days", 31);


    // Positional arguments can be used. Specifying an integer inside `{}`
    // determines which additional argument will be replaced. Arguments start
    // at 0 immediately after the format string.
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    // as a named arguments
    println!("{subject} {verb} {object}",
            object="the lazy dog",
            subject="the quick brown fox",
            verb="jumps over");

    
    // Different formatting can be invoked by specifying the format character
    // after a `:`.
    println!("Base 10:    {}", 69420);
    println!("Base 2 (binary):     {:b}", 69420);
    println!("Base 8 (octal):       {:0}", 69420);
    println!("Base 16 (hexadecimal):     {:x}", 69420);

    // You can right-justify text with a specified width. This will
    // output "    1". (Four white spaces and a "1", for a total width of 5.)
    println!("{number:>5}", number=1);

    // You can pad numbers with extra zeros
    println!("{number:0>5}", number=1);
    // and left-adjust by flipping the sign. This will output "10000".
    println!("{number:0<5}", number=1);
    // You can use named arguments in the format specifier by appending a `$`.
    println!("{number:0>width$}", number=1, width=6);

    // Rust even checks to make sure the correct number of arguments are used.
    println!("My name is {0}, {1} {0}", "Bond", "James");

    // Only types that implement fmt::Display can be formatted with `{}`. User-
    // defined types do not implement fmt::Display by default.
    #[allow(dead_code)] // disable `dead_code` which warn against unused module
    struct Structure(i32);

    // This will not compile because `Structure` does not implement
    // fmt::Display.
    // println!("This struct `{}` won't print...", Structure(3));
    // TODO ^ Try uncommenting this line


    // For Rust 1.58 and above, you can directly capture the argument from a
    // surrounding variable. Just like the above, this will output
    // "    1", 4 white spaces and a "1".
    let number: f64 = 1.0;
    let width: usize = 5;
    println!("{number:>width$}");

    let pi: f64 = 3.141592;
    let precision: usize = 3;
    println!("I is roughly {pi:.precision$}");
}