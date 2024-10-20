fn primitive_types_four() {
    let a = [1,2,3,4,5];
    let nice_slice = &a[1..4];
    println!("{}", a[2]);
    println!("{:?}", a);
    println!("slice one: {:?}", nice_slice);
    let nice_slice = &a[1..=3];
    println!("slice two: {:?}", nice_slice);
}


#[cfg(test)]
mod tests {
    #[test]
    fn slice_out_of_array() {
        let a = [1,2,3,4,5];
        let nice_slice = &a[1..4];

        assert_eq!([2,3,4], nice_slice);

        let nice_slice = &a[1..=3];
        assert_eq!([2,3,4], nice_slice);
    }
}
