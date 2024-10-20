pub fn nested_arrays_exercise() {
    let matrix = [
        [101, 102, 103],
        [201, 202, 203],
        [301, 302, 303],
    ];

    println!("matrix: {:#?}", matrix);
    let transposed = transpose(matrix);
    println!("transposed: {:#?}", transposed);
}


fn transpose(matrix: [[i32;3];3]) -> [[i32;3];3] {
    let mut result = [[0;3];3];

    for i in 0..3 {
        for j in 0..3 {
            result[i][j] = matrix[j][i]
        }
    }

    result
}



#[test]
fn test_transpose() {
    let matrix = [
        [101, 102, 103],
        [201, 202, 203],
        [301, 302, 303],
    ];

    let transposed = transpose(matrix);

    assert_eq!(transposed, 
        [
            [101, 201, 301],
            [102, 202, 302],
            [103, 203, 303],
        ]
    );
}