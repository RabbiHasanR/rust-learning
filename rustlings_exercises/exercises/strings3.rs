use std::result;

fn trim_me(input: &str) -> &str {
    // trim using two methods
    // let trim_start = input.trim_start();
    // let trim_end = trim_start.trim_end();
    // trim_end

    // trim using trim
    input.trim()
}


fn compose_me(input: &str) -> String {
    // i can do it many way
    // this is first way
    // let mut input_str = input.to_string();
    // input_str.push_str(" world!")

    // this second way
    // let result = input.to_string() + " world!";
    // result

    // this third way
    let world = " world!";
    let result = format!("{}{}", input, world);
    result
}

fn replace_me(input: &str) -> String {
    input.replace("cars", "balloons")
}


fn strings_three() {

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trim_a_string() {
        assert_eq!(trim_me("Hello!     "), "Hello!");
        assert_eq!(trim_me("   What's up!"), "What's up!");
        assert_eq!(trim_me("    Hola!    "), "Hola!");
    }

    #[test]
    fn compose_a_string() {
        assert_eq!(compose_me("Hello"), "Hello world!");
        assert_eq!(compose_me("Goodbye"), "Goodbye world!");
    }

    #[test]
    fn replace_a_string() {
        assert_eq!(
            replace_me("I think cars are cool"),
            "I think balloons are cool",
        );
        assert_eq!(
            replace_me("I love to look at cars"),
            "I love to look at balloons",
        );
    }
}