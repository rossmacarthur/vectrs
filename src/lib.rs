//! This crate provides a stack-allocated, constant-size [`Matrix<T, M, N>`]
//! type implemented using const generics.
//!
//! ### Types
//!
//! The base [`Matrix<T, M, N>`] type represents a matrix with `M` rows and `N`
//! columns. This type is a backed by an array of arrays. The data is stored in
//! column-major order. Some convenient aliases are provided for common
//! matrices, like vectors.
//!
//! - [`Matrix<T, M, N>`] → a generic matrix type with `M` rows and `N` columns.
//! - [`Vector<T, M>`] → a column vector with `M` rows.
//! - [`RowVector<T, N>`] → a row vector with `N` columns.
//!
//! ### Macros
//!
//! Macros are provided for easy construction of the provided types. These
//! macros will also work in `const` contexts.
//!
//! The [`matrix!`] macro can be used to construct a new [`Matrix`] of any size.
//! ```
//! # use vectrix::*;
//! #
//! let matrix = matrix![
//!     1, 3, 5;
//!     2, 4, 6;
//! ];
//! ```
//!
//! In the above example `matrix` is a `Matrix<_, 2, 3>` type, having 2 rows and
//! 3 columns.
//!
//! The [`vector!`] and [`row_vector!`] macros can be used to to construct
//! vectors.
//!
//! ```
//! # use vectrix::*;
//! #
//! let vector = vector![1, 3, 3, 7];
//! //  ^^^^^^ type `Vector<_, 4>`
//! assert_eq!(vector, matrix![1; 3; 3; 7]);
//!
//! let vector = row_vector![1, 3, 3, 7];
//! //  ^^^^^^ type `RowVector<_, 4>`
//! assert_eq!(vector, matrix![1, 3, 3, 7]);
//! ```
//!
//! ### Constructors
//!
//! Commonly used constructors are listed below.
//!
//! - [`::default()`][`Matrix::default()`] → constructs a new matrix filled with
//!   [`T::default()`][`Default::default()`].
//! - [`::zero()`][`Matrix::zero()`] → constructs a new matrix filled with
//!   [`T::zero()`][`Zero::zero()`].
//! - [`::identity()`][`Matrix::identity()`] → constructs a new identity matrix.
//! - [`::repeat(..)`][`Matrix::repeat()`] → constructs a new matrix filled with
//!   the provided value.
//! - [`::repeat_with(..)`][`Matrix::repeat_with()`] → constructs a new matrix
//!   filled with values from the provided closure.
//! - [`::from_iter(..)`][`FromIterator::from_iter`] → constructs a new matrix
//!   from an iterator. See also [`collect()`][`Iterator::collect()`].
//! - [`::new(..)`][`Matrix::new()`] → constructs a new vector using the
//!   provided components.
//!
//! ### Accessing elements
//!
//! Two types of indexing is available:
//!
//! Firstly, `usize` indexing which selects the nth element in the matrix as
//! viewed in column-major order.
//! ```
//! # use vectrix::*;
//! #
//! let matrix = matrix![
//!     1, 2, 3;
//!     4, 5, 6;
//! ];
//! assert_eq!(matrix[1], 4);
//! ```
//!
//! Secondly, `(usize, usize)` indexing which selects the element at a
//! particular row and column position.
//! ```
//! # use vectrix::*;
//! #
//! let matrix = matrix![
//!     1, 2, 3;
//!     4, 5, 6;
//! ];
//! assert_eq!(matrix[(1, 0)], 4);
//! ```
//!
//! Additionally, component accessors are available for small vectors using
//! commonly recognized names.
//! ```
//! # use vectrix::*;
//! #
//! let mut vector = vector![1, 2, 3, 4, 0, 0];
//! vector.y = 3;
//! vector.w = 7;
//! assert_eq!(vector.x, 1);
//! assert_eq!(vector.y, 3);
//! assert_eq!(vector.z, 3);
//! assert_eq!(vector.w, 7);
//! assert_eq!(vector.a, 0);
//! assert_eq!(vector.b, 0);
//! ```
//!
//! ### Accessing a row or column
//!
//! You can get a reference to particular row or column using the
//! [`.row()`][`Matrix::row`] or [`.column()`][`Matrix::column`] methods.
//!
//! ```
//! # use vectrix::*;
//! #
//! let mut matrix = matrix![
//!     1, 2, 3;
//!     4, 7, 6;
//! ];
//! let row = matrix.row_mut(1);
//! row[1] = 5;
//! assert_eq!(matrix.column(1), &[2, 5]);
//! ```
//!
//! ### Iteration
//!
//! Element-wise, column-major order iteration is provided using the following
//! methods.
//!
//! - [`.into_iter()`][`Matrix::into_iter()`] → consumes the matrix and returns
//!   an owned iterator over each element.
//! - [`.iter()`][`Matrix::iter()`] → returns an iterator over a reference to
//!   each element.
//! - [`.iter_mut()`][`Matrix::iter_mut()`] → returns an iterator over a mutable
//!   reference to each element.
//!
//! Iteration over rows and columns is provide using the following methods.
//!
//! - [`.iter_rows()`][`Matrix::iter_rows()`] → returns an iterator over a
//!   reference to each row.
//! - [`.iter_rows_mut()`][`Matrix::iter_rows_mut()`] → returns an iterator over
//!   mutable reference to each row.
//! - [`.iter_columns()`][`Matrix::iter_columns()`] → returns an iterator over a
//!   reference to each column.
//! - [`.iter_columns_mut()`][`Matrix::iter_columns_mut()`] → returns an
//!   iterator over a mutable reference to each column.
//!
//! ### Slice representation
//!
//! A slice view of the underlying data is provided using
//! [`.as_slice()`][`Matrix::as_slice`] and
//! [`.as_mut_slice()`][`Matrix::as_mut_slice`].
//! ```
//! # use vectrix::*;
//! #
//! let mut matrix = matrix![
//!     1, 3, 5;
//!     2, 3, 6;
//! ];
//! matrix.as_mut_slice()[3] = 4;
//! assert_eq!(matrix.as_slice(), &[1, 2, 3, 4, 5, 6]);
//! ```
//!
//! ### Operations
//!
//! [`Matrix`] implements many built-in operators. With scalar operands almost
//! all operators are implemented and they simply apply the operation to each
//! element in the matrix. Unary operators will do the equivalent. In the
//! following example each element in the matrix is multiplied by 2.
//!
//! ```
//! # use vectrix::*;
//! #
//! let matrix = matrix![
//!     1, -3;
//!     3, -7;
//! ];
//! let expected = matrix![
//!     2, -6;
//!     6, -14;
//! ];
//! assert_eq!(matrix * 2, expected);
//! ```
//!
//! [`Matrix`] supports addition and subtraction with same size matrices for
//! element-wise addition and subtraction. In the following example a matrix
//! is added to itself.
//!
//! ```
//! # use vectrix::*;
//! #
//! let matrix = matrix![
//!     1, -3;
//!     3, -7;
//! ];
//! let expected = matrix![
//!     2, -6;
//!     6, -14;
//! ];
//! assert_eq!(matrix + matrix, expected);
//! ```

#![no_std]

#[cfg(feature = "std")]
extern crate std;

mod fmt;
mod iter;
mod ops;
mod prelude;
pub mod traits;
mod vector;
mod view;

use core::iter::{repeat_with, FromIterator, Sum};
use core::slice;

#[doc(hidden)]
#[cfg(feature = "macro")]
pub use vectrix_macro as proc_macro;

pub use crate::iter::{IntoIter, IterColumns, IterColumnsMut, IterRows, IterRowsMut};
pub use crate::view::{Column, Row};

use crate::prelude::*;

/// Represents a matrix with constant `M` rows and constant `N` columns.
///
/// The underlying data is represented as an array and is always stored in
/// column-major order.
///
/// See the [crate root][crate] for usage examples.
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(transparent)]
pub struct Matrix<T, const M: usize, const N: usize> {
    data: [[T; M]; N],
}

/// A matrix with one row and `N` columns.
pub type RowVector<T, const N: usize> = Matrix<T, 1, N>;

/// A matrix with one column and `M` rows.
pub type Vector<T, const M: usize> = Matrix<T, M, 1>;

////////////////////////////////////////////////////////////////////////////////
// Constructors
////////////////////////////////////////////////////////////////////////////////

/// A macro for composing matrices.
///
/// This macro allows one to write such a matrix in the natural order. For
/// example:
///
/// ```rust
/// # use vectrix::matrix;
/// #
/// let m = matrix![
///     1.0, 4.0;
///     2.0, 5.0;
///     3.0, 6.0;
/// ];
/// ```
///
/// corresponds to the following matrix with three rows and two columns:
///
/// ```text
/// ┌            ┐
/// │  1.0  4.0  │
/// │  2.0  5.0  │
/// │  3.0  6.0  │
/// └            ┘
/// ```
///
/// Which is stored as an array of arrays in column-major order.
///
/// ```text
/// Matrix { data: [[1.0, 2.0, 3.0], [4.0, 5.0, 6.0]] }
/// ```
#[cfg(feature = "macro")]
#[macro_export]
macro_rules! matrix {
    ($($data:tt)*) => {
        $crate::Matrix::from_column_major_order($crate::proc_macro::matrix!($($data)*))
    };
}

impl<T, const M: usize, const N: usize> Default for Matrix<T, M, N>
where
    T: Copy + Default,
{
    /// Create a new matrix using `T::default()` as an initializer.
    #[inline]
    fn default() -> Self {
        Self::repeat(T::default())
    }
}

impl<T, const M: usize, const N: usize> FromIterator<T> for Matrix<T, M, N>
where
    T: Copy + Default,
{
    /// Create a new matrix from an iterator. Elements will be filled in
    /// column-major order.
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        let mut iter = iter.into_iter();
        let mut matrix = Self::default();
        for idx in 0..(M * N) {
            match iter.next() {
                Some(value) => matrix[idx] = value,
                None => {
                    panic!(
                        "collect iterator of length {} into `Matrix<_, {}, {}>`",
                        idx, M, N
                    );
                }
            }
        }
        matrix
    }
}

////////////////////////////////////////////////////////////////////////////////
// Matrix<T, M, N> methods
////////////////////////////////////////////////////////////////////////////////

impl<T, const M: usize, const N: usize> Matrix<T, M, N> {
    /// Create a new matrix from an array of arrays in column-major order.
    #[doc(hidden)]
    #[inline]
    pub const fn from_column_major_order(data: [[T; M]; N]) -> Self {
        Self { data }
    }

    /// Returns a zero matrix.
    #[must_use]
    #[inline]
    pub fn zero() -> Self
    where
        T: Copy + Zero,
    {
        Self::repeat(T::zero())
    }

    /// Create a new matrix filled with the given element.
    #[must_use]
    #[inline]
    pub fn repeat(element: T) -> Self
    where
        T: Copy,
    {
        Self {
            data: [[element; M]; N],
        }
    }

    /// Create a new matrix filled with elements computed from the given closure.
    #[inline]
    pub fn repeat_with<F>(repeater: F) -> Self
    where
        T: Copy + Default,
        F: FnMut() -> T,
    {
        repeat_with(repeater).collect()
    }

    /// Views the underlying data as a contiguous slice.
    #[inline]
    pub fn as_slice(&self) -> &[T] {
        let ptr = self.data.as_ptr() as *const T;
        unsafe { slice::from_raw_parts(ptr, M * N) }
    }

    /// Views the underlying data as a contiguous mutable slice.
    #[inline]
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        let ptr = self.data.as_mut_ptr() as *mut T;
        unsafe { slice::from_raw_parts_mut(ptr, M * N) }
    }

    /// Returns a reference to the `i`-th row of this matrix.
    #[inline]
    pub fn row(&self, i: usize) -> &Row<T, M, N> {
        Row::new(&self.as_slice()[i..])
    }

    /// Returns a mutable reference to the `i`-th row of this matrix.
    #[inline]
    pub fn row_mut(&mut self, i: usize) -> &mut Row<T, M, N> {
        Row::new_mut(&mut self.as_mut_slice()[i..])
    }

    /// Returns a reference to the `i`-th column of this matrix.
    #[inline]
    pub fn column(&self, i: usize) -> &Column<T, M, N> {
        Column::new(&self.data[i])
    }

    /// Returns a mutable reference to the `i`-th column of this matrix.
    #[inline]
    pub fn column_mut(&mut self, i: usize) -> &mut Column<T, M, N> {
        Column::new_mut(&mut self.data[i])
    }

    /// Returns an iterator over the underlying data.
    #[inline]
    pub fn iter(&self) -> slice::Iter<'_, T> {
        self.as_slice().iter()
    }

    /// Returns a mutable iterator over the underlying data.
    #[inline]
    pub fn iter_mut(&mut self) -> slice::IterMut<'_, T> {
        self.as_mut_slice().iter_mut()
    }

    /// Returns an iterator over the rows in this matrix.
    #[inline]
    pub fn iter_rows(&self) -> IterRows<'_, T, M, N> {
        IterRows::new(self)
    }

    /// Returns a mutable iterator over the rows in this matrix.
    #[inline]
    pub fn iter_rows_mut(&mut self) -> IterRowsMut<'_, T, M, N> {
        IterRowsMut::new(self)
    }

    /// Returns an iterator over the columns in this matrix.
    #[inline]
    pub fn iter_columns(&self) -> IterColumns<'_, T, M, N> {
        IterColumns::new(self)
    }

    /// Returns a mutable iterator over the columns in this matrix.
    #[inline]
    pub fn iter_columns_mut(&mut self) -> IterColumnsMut<'_, T, M, N> {
        IterColumnsMut::new(self)
    }

    /// Returns a matrix of the same size as self, with function `f` applied to
    /// each element in order.
    #[inline]
    pub fn map<F, U>(self, mut f: F) -> Matrix<U, M, N>
    where
        T: Copy,
        U: Copy + Default,
        F: FnMut(T) -> U,
    {
        let mut matrix = Matrix::default();
        for idx in 0..(M * N) {
            matrix[idx] = f(self[idx]);
        }
        matrix
    }

    /// Returns the L1 norm of the matrix.
    ///
    /// Also known as *Manhattan Distance* or *Taxicab norm*. L1 Norm is the sum
    /// of the magnitudes of the vectors in a space.
    ///
    /// Note: if the matrix is a *row vector* this method might not do what you
    /// what you expect. For example:
    ///
    /// ```
    /// # use vectrix::matrix;
    /// #
    /// let row_vector = matrix![1, 2, 3];
    /// assert_eq!(row_vector.l1_norm(), 3);
    ///
    /// let column_vector = matrix![1; 2; 3];
    /// assert_eq!(column_vector.l1_norm(), 6);
    /// ```
    pub fn l1_norm(&self) -> T
    where
        T: Copy + Ord + Abs + Zero + Sum<T>,
    {
        (0..N)
            .map(|idx| self.data[idx].iter().copied().map(Abs::abs).sum())
            .max()
            .unwrap_or_else(Zero::zero)
    }
}

////////////////////////////////////////////////////////////////////////////////
// Matrix<T, N, N> methods
////////////////////////////////////////////////////////////////////////////////

impl<T, const N: usize> Matrix<T, N, N> {
    /// Returns an identity matrix.
    #[must_use]
    #[inline]
    pub fn identity() -> Self
    where
        T: Copy + Default + One + Zero,
    {
        let mut matrix = Self::zero();
        for idx in 0..N {
            matrix[(idx, idx)] = T::one();
        }
        matrix
    }

    /// Returns the diagonal of the matrix.
    pub fn diagonal(&self) -> Vector<T, N>
    where
        T: Copy + Default,
    {
        let mut vector = Vector::default();
        for idx in 0..N {
            vector[idx] = self[(idx, idx)];
        }
        vector
    }
}
