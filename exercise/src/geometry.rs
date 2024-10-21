fn magnitude(vector: &[f64; 3]) -> f64 {
    let mut mag_squared = 0.0;
    for coord in vector {
        mag_squared += coord * coord;
    }
    mag_squared
}

fn normalize(vector: &mut [f64; 3]) {
    let mag = magnitude(vector);
    for item in vector {
        *item /= mag;
    }
}


pub fn geometry() {
    println!("Magnitude of a unit vector: {}", magnitude(&[0.0,1.0,0.0]));
    let mut v = [1.0,2.0,9.0];

    println!("Magnitude of {v:?}: {}", magnitude(&v));
    normalize(&mut v);
    println!("After normalization {v:?}");
    println!("Magniture of {v:?} after normalization: {}", magnitude(&v));

}