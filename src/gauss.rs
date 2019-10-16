use std::ops::{Add, Mul, Index, IndexMut};
use crate::zero::Zero;

pub struct Mat<'a, T> {
    slice: &'a mut [T],
    row_size: usize,
}

impl<'a, T> Index<(usize, usize)> for Mat<'a, T> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        return &self.slice[self.row_size * index.0 + index.1];
    }
}

impl<'a, T> IndexMut<(usize, usize)> for Mat<'a, T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        return &mut self.slice[self.row_size * index.0 + index.1];
    }
}

impl<'a, T> Mat<'a, T> where T: Add + Mul<Output=T> + Copy {
    pub fn new(slice: &'a mut [T], row_size: usize) -> Self {
        // make sure the slice is a rectangle
        if slice.len() % row_size != 0 { panic!("slice.len() & row_size != 0") }

        return Mat { slice, row_size };
    }

    pub fn shape(&self) -> (usize, usize) {
        return (self.slice.len() / self.row_size, self.row_size);
    }

    pub fn mul_row(&mut self, row: usize, f: T) {
        for col in 0..self.row_size {
            self[(row, col)] = self[(row, col)] * f;
        }
    }

    pub fn swap_rows(&mut self, row1: usize, row2: usize) {
        for col in 0..self.row_size {
            self.slice.swap(self.row_size * row1 + col, self.row_size * row2 + col);
        }
    }
}

pub fn gauss_elimination<T>(mat: Mat<T>) where T: PartialEq + Zero + Add + Mul<Output=T> + Copy {
    let (rows, cols) = mat.shape();


    // for each column we find a first row with non-zero pivot
    // then we make the pivot equal to one by dividing the row
    // with pivot's value.
    for col in 0..cols {

        // find first non-zero pivot
        let pivot = {
            let mut r = None;
            for row in col..rows {
                if mat[(row, col)] != T::zero() {
                    r = Some(row);
                    break;
                }
            };
            r.unwrap_or_else(|| panic!("mat has no non-zero pivot for column {}", col))
        };
    }
}