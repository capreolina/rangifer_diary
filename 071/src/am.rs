use std::{
    collections::HashSet,
    ops::{Index, IndexMut},
};

pub struct AdjacencyMatrix {
    stride: usize,
    data: Vec<HashSet<String>>,
}

impl AdjacencyMatrix {
    pub fn empty(stride: usize) -> Self {
        Self {
            stride,
            data: vec![HashSet::new(); stride * stride],
        }
    }

    pub fn stride(&self) -> usize {
        self.stride
    }
}

impl Index<(usize, usize)> for AdjacencyMatrix {
    type Output = HashSet<String>;

    fn index(&self, (i, j): (usize, usize)) -> &Self::Output {
        &self.data[i * self.stride + j]
    }
}

impl IndexMut<(usize, usize)> for AdjacencyMatrix {
    fn index_mut(&mut self, (i, j): (usize, usize)) -> &mut Self::Output {
        &mut self.data[i * self.stride + j]
    }
}
