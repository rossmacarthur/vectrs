mod fmt;
mod iter;
mod ops;

use core::iter::FromIterator;

use vectrix::{matrix, vector, Matrix};

////////////////////////////////////////////////////////////////////////////////
// Constructors
////////////////////////////////////////////////////////////////////////////////

#[test]
fn matrix_macro_const() {
    const MATRIX: Matrix<i64, 2, 2> = matrix![1, 3; 3, 7];
    assert_eq!(MATRIX, matrix![1, 3; 3, 7]);
}

#[test]
fn matrix_macro_row_vector() {
    let _matrix: Matrix<i64, 1, 4> = matrix![1, 3, 3, 7];
}

#[test]
fn matrix_macro_vector() {
    let _matrix: Matrix<i64, 4, 1> = matrix![1; 3; 3; 7];
}

#[test]
fn matrix_default() {
    let matrix = Matrix::default();
    assert_eq!(matrix, matrix![0, 0; 0, 0]);
}

#[test]
fn matrix_from_iter() {
    let matrix = Matrix::<i64, 2, 2>::from_iter(vec![1, 2, 3, 4]);
    assert_eq!(matrix, matrix![1, 3; 2, 4]);
}

#[test]
fn matrix_from_iter_long() {
    let matrix = Matrix::<i64, 2, 2>::from_iter(vec![1, 2, 3, 4, 5]);
    assert_eq!(matrix, matrix![1, 3; 2, 4]);
}

#[test]
#[should_panic]
fn matrix_from_iter_short() {
    let _matrix = Matrix::<i64, 2, 2>::from_iter(vec![1, 2, 3]);
}

////////////////////////////////////////////////////////////////////////////////
// Matrix<T, M, N> methods
////////////////////////////////////////////////////////////////////////////////

#[test]
fn matrix_zero() {
    let matrix = Matrix::zero();
    assert_eq!(matrix, matrix![0, 0; 0, 0]);
}

#[test]
fn matrix_repeat() {
    let matrix = Matrix::repeat(7);
    assert_eq!(matrix, matrix![7, 7; 7, 7]);
}

#[test]
fn matrix_repeat_with() {
    let mut state = 1;
    let matrix = Matrix::repeat_with(|| {
        state *= 2;
        state
    });
    assert_eq!(matrix, matrix![2, 8; 4, 16]);
}

#[test]
fn matrix_as_slice() {
    let matrix = matrix![1, 3, 3, 7];
    assert_eq!(matrix.as_slice(), vec![1, 3, 3, 7].as_slice());
}

#[test]
fn matrix_as_mut_slice() {
    let mut matrix = matrix![1, 2; 3, 4];
    let slice = matrix.as_mut_slice();
    slice[2] = 3;
    slice[3] = 7;
    assert_eq!(matrix, matrix![1, 3; 3, 7]);
}

#[test]
fn matrix_iter() {
    let matrix = matrix![1, 3; 2, 4];
    let values: Vec<_> = matrix.iter().collect();
    assert_eq!(values, vec![&1, &2, &3, &4]);
}

#[test]
fn matrix_iter_mut() {
    let mut matrix = matrix![0, 2; 2, 6];
    for n in matrix.iter_mut() {
        *n += 1;
    }
    assert_eq!(matrix, matrix![1, 3; 3, 7]);
}

#[test]
fn matrix_row() {
    let matrix = matrix![1, 3; -3, 7];
    let row = matrix.row(0);
    let vec: Vec<_> = row.iter().collect();
    assert_eq!(vec, &[&1, &3]);
    let row = matrix.row(1);
    let vec: Vec<_> = row.iter().collect();
    assert_eq!(vec, &[&-3, &7]);
}

#[test]
fn matrix_row_mut() {
    let mut matrix = matrix![2, 3; -3, 8];
    let row = matrix.row_mut(0);
    row[0] = 1;
    let row = matrix.row_mut(1);
    row[1] = 7;
    assert_eq!(matrix, matrix![1, 3; -3, 7]);
}

#[test]
fn matrix_column() {
    let matrix = matrix![1, 3; -3, 7];
    let column = matrix.column(0);
    let vec: Vec<_> = column.iter().collect();
    assert_eq!(vec, &[&1, &-3]);
    let column = matrix.column(1);
    let vec: Vec<_> = column.iter().collect();
    assert_eq!(vec, &[&3, &7]);
}

#[test]
fn matrix_column_mut() {
    let mut matrix = matrix![2, 3; -3, 8];
    let column = matrix.column_mut(0);
    column[0] = 1;
    let column = matrix.column_mut(1);
    column[1] = 7;
    assert_eq!(matrix, matrix![1, 3; -3, 7]);
}

#[test]
fn matrix_l1_norm() {
    let matrix = matrix![-1, 3; -3, 7];
    assert_eq!(matrix.l1_norm(), 10);
}

#[test]
fn matrix_l1_norm_vectors() {
    let matrix = matrix![-1, 3, -3, 7];
    assert_eq!(matrix.l1_norm(), 7);

    let matrix = matrix![-1; 3; -3; 7];
    assert_eq!(matrix.l1_norm(), 14);
}

////////////////////////////////////////////////////////////////////////////////
// Matrix<T, N, N> methods
////////////////////////////////////////////////////////////////////////////////

#[test]
fn matrix_identity() {
    assert_eq!(
        Matrix::identity(),
        matrix![
            1, 0;
            0, 1;
        ]
    );

    assert_eq!(
        Matrix::identity(),
        matrix![
            1, 0, 0;
            0, 1, 0;
            0, 0, 1;
        ]
    );

    assert_eq!(
        Matrix::identity(),
        matrix![
            1, 0, 0, 0;
            0, 1, 0, 0;
            0, 0, 1, 0;
            0, 0, 0, 1;
        ]
    );
}

#[test]
fn matrix_diagonal() {
    let matrix = matrix![
        1, 0, 0;
        0, 2, 0;
        0, 0, 3;
    ];
    assert_eq!(matrix.diagonal(), vector![1, 2, 3]);
}
