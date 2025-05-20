use core::{isize, usize};

use crate::math::F32x2;
pub use heapless::{FnvIndexMap, Vec};

pub trait NeighborSearcher {
    type Point;
    fn get_neigbors(&self, point: Self::Point) -> impl Iterator<Item = usize>;
    fn build(&mut self, points: &[Self::Point]);
}
#[derive(Debug)]
pub struct ParticleSearchStaticMatrix<const C: usize, const PPC: usize> {
    pub cells: [Vec<usize, PPC>; C],
    width: usize,
    cel_size: f32,
}

impl<const C: usize, const PPC: usize> ParticleSearchStaticMatrix<C, PPC> {
    pub fn new(width: usize, cel_size: f32) -> Self {
        Self {
            cells: [const { Vec::new() }; C],
            width,
            cel_size,
        }
    }
    #[inline]
    pub fn get_grid_index(&self, point: &F32x2) -> (usize, usize) {
        (
            (point.x / self.cel_size) as usize,
            (point.y / self.cel_size) as usize,
        )
    }
    #[inline]
    pub fn get_cell_index(&self, point: &F32x2) -> usize {
        (point.x / self.cel_size) as usize + (point.y / self.cel_size) as usize * self.width
    }
    #[inline]
    fn get_grid_indexes(&self, coord: (isize, isize), height: usize) -> Option<(usize, usize)> {
        if self.is_valid_grid(coord, height) {
            Some((coord.0 as usize, coord.1 as usize))
        } else {
            None
        }
    }
    #[inline]
    pub fn is_valid_grid(&self, coord: (isize, isize), height: usize) -> bool {
        coord.0 >= 0 && coord.0 < self.width as isize && coord.1 >= 0 && coord.1 < height as isize
    }
    #[inline]
    pub fn gen_valid_cell_indexes(&self, point: F32x2) -> impl Iterator<Item = (usize, usize)> {
        let grid_i = self.get_grid_index(&point);
        let height = C / self.width;
        let iter = (-1..=1).flat_map(move |i| {
            (-1..=1).flat_map(move |j| {
                self.get_grid_indexes((grid_i.0 as isize + i, grid_i.1 as isize + j), height)
            })
        });
        iter
    }
}
impl<const C: usize, const PPC: usize> NeighborSearcher for ParticleSearchStaticMatrix<C, PPC> {
    type Point = F32x2;
    fn build(&mut self, points: &[Self::Point]) {
        self.cells = [const { Vec::new() }; C];
        for (point_i, point) in points.iter().enumerate() {
            let cell_i = self.get_cell_index(point);
            if let Err(_e) = self.cells[cell_i].push(point_i) {}
        }
    }
    fn get_neigbors(&self, point: Self::Point) -> impl Iterator<Item = usize> {
        let iter = self.gen_valid_cell_indexes(point);
        iter.flat_map(|(x, y)| &self.cells[x + y * self.width])
            .map(|&n| n)
    }
}
