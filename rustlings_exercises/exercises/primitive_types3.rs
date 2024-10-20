fn primitive_types_three() {
    // let a = [1,2,3,4,5,6,7,8,9,10];
    let a = [42; 100];

    if a.len() >= 100 {
        println!("Wow, that's a big array!");
        println!("{:?}", a);
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
        println!("Array not big enough, more elements needed");
    }
}