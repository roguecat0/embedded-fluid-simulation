use core::{isize, usize};

use super::PARTICLES;
use crate::math::F32x2;
pub use heapless::{FnvIndexMap, Vec};

const MAP_SIZE: usize = PARTICLES.next_power_of_two();

pub trait NeighborSearcher {
    type Point;
    fn get_neigbors(&self, point: Self::Point) -> impl Iterator<Item = usize>;
    fn build(&mut self, points: &[Self::Point]);
}
pub struct ParticleSearchStaticMatrix<const C: usize, const PPC: usize> {
    pub cells: Vec<Vec<usize, PPC>, C>,
    width: usize,
    cel_size: f32,
}

impl<const C: usize, const PPC: usize> ParticleSearchStaticMatrix<C, PPC> {
    pub fn new(width: usize, cel_size: f32) -> Self {
        Self {
            cells: Vec::new(),
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
    fn get_cell_index_if_valid_grid(&self, coord: (isize, isize), height: usize) -> Option<usize> {
        if coord.0 > 0 && coord.0 < self.width as isize && coord.1 > 0 && coord.1 < height as isize
        {
            Some(coord.0 as usize + coord.1 as usize * self.width)
        } else {
            None
        }
    }
}
impl<const C: usize, const PPC: usize> NeighborSearcher for ParticleSearchStaticMatrix<C, PPC> {
    type Point = F32x2;
    fn build(&mut self, points: &[Self::Point]) {
        for (point_i, point) in points.iter().enumerate() {
            let cell_i = self.get_cell_index(point);
            if let Err(e) = self.cells[cell_i].push(point_i) {}
        }
    }
    fn get_neigbors(&self, point: Self::Point) -> impl Iterator<Item = usize> {
        let grid_i = self.get_grid_index(&point);
        let height = C / self.width;
        let iter = (-1..1).flat_map(move |i| {
            (-1..1).flat_map(move |j| {
                self.get_cell_index_if_valid_grid(
                    (grid_i.0 as isize + i, grid_i.1 as isize + j),
                    height,
                )
            })
        });
        iter.flat_map(|n| &self.cells[n]).map(|&n| n)
    }
}

// alternative with fixed dimentions you can just x + width * y
pub struct ParticleSearchHeapless {
    pub spacial_lookup: [(usize, usize); PARTICLES],
    pub start_index_map: FnvIndexMap<(usize, usize), usize, MAP_SIZE>,
    grid_spacing: f32,
}
impl ParticleSearchHeapless {
    pub fn new(grid_spacing: f32) -> Self {
        Self {
            spacial_lookup: [(0, 0); PARTICLES],
            start_index_map: FnvIndexMap::new(),
            grid_spacing,
        }
    }
    #[inline]
    fn sort_spacial_lookup(&mut self) {
        self.spacial_lookup.sort_unstable();
    }
    #[inline]
    pub fn get_grid_index(&self, point: &F32x2) -> (usize, usize) {
        (
            (point.x / self.grid_spacing) as usize,
            (point.y / self.grid_spacing) as usize,
        )
    }
    #[inline]
    fn get_points_in_grid(&self, start_index: usize) -> impl Iterator<Item = usize> + use<'_> {
        let start_cel = self.spacial_lookup[start_index];
        self.spacial_lookup[start_index..]
            .iter()
            .enumerate()
            .map_while(move |(i, cel)| (start_cel == *cel).then_some(i))
    }
    pub fn get_cells(&self, point: F32x2) -> Vec<(isize, isize), 9> {
        let index = self.get_grid_index(&point);
        let mut v = Vec::<(isize, isize), 9>::new();
        // optimization: add more checks add edges of radius
        for i in -1..=1 {
            for j in -1..=1 {
                v.push((index.0 as isize + i, index.1 as isize + j))
                    .unwrap()
            }
        }
        v
    }
}
impl NeighborSearcher for ParticleSearchHeapless {
    type Point = F32x2;
    fn build(&mut self, points: &[Self::Point]) {
        for (i, point) in points.iter().enumerate() {
            self.spacial_lookup[i] = self.get_grid_index(point);
        }
        self.sort_spacial_lookup();
        let mut prev = (usize::MAX, usize::MAX);
        for (i, cel) in self.spacial_lookup.iter().enumerate() {
            if &prev != cel {
                self.start_index_map.insert(*cel, i).unwrap();
                prev = *cel;
            }
        }
    }
    fn get_neigbors(&self, point: Self::Point) -> impl Iterator<Item = usize> {
        let index = self.get_grid_index(&point);
        let mut v = Vec::<usize, 9>::new();
        // optimization: add more checks add edges of radius
        for i in -1..=1 {
            for j in -1..=1 {
                if let Some(start_index) = self.start_index_map.get(&(
                    (index.0 as isize + i) as usize,
                    (index.1 as isize + j) as usize,
                )) {
                    v.push(*start_index).unwrap();
                }
            }
        }
        v.into_iter()
            .flat_map(|index| self.get_points_in_grid(index))
    }
}
