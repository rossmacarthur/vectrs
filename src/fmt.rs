#![cfg(feature = "std")]

use core::fmt;

use crate::prelude::*;

fn fmt_row<I, T, F>(
    f: &mut fmt::Formatter<'_>,
    left: &str,
    right: &str,
    widths: &[usize],
    data: I,
    mut fmt: F,
) -> fmt::Result
where
    I: Iterator<Item = T>,
    F: FnMut(&mut fmt::Formatter<'_>, T, usize) -> fmt::Result,
{
    f.write_str(left)?;
    for (i, (d, &width)) in data.zip(widths).enumerate() {
        if i != 0 {
            f.write_str(", ")?;
        }
        fmt(f, d, width)?;
    }
    f.write_str(right)?;
    Ok(())
}

fn fmt_matrix<T, F1, F2, const M: usize, const N: usize>(
    matrix: &Matrix<T, M, N>,
    f: &mut fmt::Formatter<'_>,
    width: F1,
    mut fmt: F2,
) -> fmt::Result
where
    F1: FnMut(&T) -> usize + Copy,
    F2: FnMut(&mut fmt::Formatter<'_>, &T, usize) -> fmt::Result + Copy,
{
    if M == 1 || N == 1 {
        f.write_str("(")?;
        for (i, d) in matrix.iter().enumerate() {
            if i != 0 {
                f.write_str(", ")?;
            }
            fmt(f, d, 0)?
        }
        f.write_str(")")?;
    } else {
        let widths: Vec<_> = matrix
            .iter_columns()
            .map(|col| col.iter().map(width).max().unwrap_or(0))
            .collect();
        for (i, row) in matrix.iter_rows().enumerate() {
            if i == 0 {
                fmt_row(f, "⎛ ", " ⎞\n", &widths, row.iter(), fmt)?;
            } else if i == (M - 1) {
                fmt_row(f, "⎝ ", " ⎠", &widths, row.iter(), fmt)?;
            } else {
                fmt_row(f, "⎜ ", " ⎟\n", &widths, row.iter(), fmt)?;
            }
        }
    }
    Ok(())
}

impl<T: fmt::Debug, const M: usize, const N: usize> fmt::Debug for Matrix<T, M, N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let precision = f.precision();
        fmt_matrix(
            self,
            f,
            |d| match precision {
                Some(places) => std::format!("{:.1$?}", d, places).len(),
                None => std::format!("{:?}", d).len(),
            },
            |f, d, width| match precision {
                Some(places) => write!(f, "{:1$.2$?}", d, width, places),
                None => write!(f, "{:1$?}", d, width),
            },
        )
    }
}

impl<T: fmt::Display, const M: usize, const N: usize> fmt::Display for Matrix<T, M, N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let precision = f.precision();
        fmt_matrix(
            self,
            f,
            |d| match precision {
                Some(places) => std::format!("{:.1$}", d, places).len(),
                None => std::format!("{}", d).len(),
            },
            |f, d, width| match precision {
                Some(places) => write!(f, "{:1$.2$}", d, width, places),
                None => write!(f, "{:1$}", d, width),
            },
        )
    }
}
