use vectrix::{matrix, row_vector, vector};

#[test]
fn vector_debug() {
    assert_eq!(format!("{:?}", vector![24, 7]), "(24, 7)");
}

#[test]
fn row_vector_debug() {
    assert_eq!(format!("{:?}", row_vector![24, 7]), "(24, 7)");
}

#[test]
fn matrix_debug() {
    assert_eq!(
        format!("{:?}", matrix![-1, 3, 0; 0, 0, 0; -3, 24, 7]),
        "⎛ -1,  3, 0 ⎞\n\
         ⎜  0,  0, 0 ⎟\n\
         ⎝ -3, 24, 7 ⎠"
    );
}

#[test]
fn matrix_debug_precision() {
    assert_eq!(
        format!("{:.2?}", matrix![-1.0, 3.12, 0.0; -3.3, 24.7839, 7.1]),
        "⎛ -1.00,  3.12, 0.00 ⎞\n\
         ⎝ -3.30, 24.78, 7.10 ⎠"
    );
}

#[test]
fn vector_display() {
    assert_eq!(format!("{}", vector![24, 7]), "(24, 7)");
}

#[test]
fn row_vector_display() {
    assert_eq!(format!("{}", row_vector![24, 7]), "(24, 7)");
}

#[test]
fn matrix_display() {
    assert_eq!(
        format!("{}", matrix![-1, 3, 0; -3, 24, 7]),
        "⎛ -1,  3, 0 ⎞\n\
         ⎝ -3, 24, 7 ⎠"
    );
}

#[test]
fn matrix_display_precision() {
    assert_eq!(
        format!("{:.2?}", matrix![-1.0, 3.12, 0.0; -3.3, 24.7839, 7.1]),
        "⎛ -1.00,  3.12, 0.00 ⎞\n\
         ⎝ -3.30, 24.78, 7.10 ⎠"
    );
}
