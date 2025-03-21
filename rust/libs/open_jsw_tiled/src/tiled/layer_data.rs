use std::ops::{Index, IndexMut};

pub struct TileMatrix<'a, T> {
    data: &'a mut Vec<T>,
    cols: usize,
}

impl<'a, T> TileMatrix<'a, T> {
    pub fn wrap_vec(data: &'a mut Vec<T>, cols: usize) -> Self
    where
        T: Default + Clone,
    {
        Self { data, cols }
    }

    pub fn get(&self, col: usize, row: usize) -> Option<&T> {
        let index = self.cols * row + col;
        self.data.get(index)
    }
}

#[allow(clippy::needless_lifetimes)]
impl<'a, T> Index<usize> for TileMatrix<'a, T> {
    type Output = [T];

    fn index(&self, index: usize) -> &Self::Output {
        let start = self.cols * index;
        &self.data[start..start + self.cols]
    }
}

#[allow(clippy::needless_lifetimes)]
impl<'a, T> IndexMut<usize> for TileMatrix<'a, T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let start = self.cols * index;
        &mut self.data[start..start + self.cols]
    }
}
