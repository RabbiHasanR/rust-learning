fn primitive_types_six() {
    let numbers = (1,2,3);
    let second = numbers.1;
    println!("Tuple second element {second}");
}

#[cfg(test)]

mod tests {
    #[test]
    fn indexing_tuple() {
        let numbers = (1,2,3);
        let second = numbers.1;
        assert_eq!(second, 2, "This is not the 2nd number in the tuple!");
    }
}