use core::usize;

use super::PARTICLES;
use crate::math::{F32x2, VectorExt};
pub use heapless::{FnvIndexMap, Vec};

const MAP_SIZE: usize = PARTICLES.next_power_of_two();

pub trait NeighborSearcher {
    type Point;
    type RadiusSize;
    fn get_neigbors(
        &self,
        radius: Self::RadiusSize,
        point: Self::Point,
    ) -> impl Iterator<Item = usize>;
    fn build(&mut self, points: &[Self::Point]);
}

// alternative with fixed dimentions you can just x + width * y
pub struct ParticleSearchHeapless {
    dimensions: (usize, usize),
    spacial_lookup: [(usize, usize); PARTICLES],
    start_index_map: FnvIndexMap<(usize, usize), usize, MAP_SIZE>,
    grid_spacing: f32,
}
impl ParticleSearchHeapless {
    #[inline]
    fn sort_spacial_lookup(&mut self) {
        self.spacial_lookup.sort_unstable();
    }
    #[inline]
    fn get_grid_index(&self, point: &F32x2) -> (usize, usize) {
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
}
impl NeighborSearcher for ParticleSearchHeapless {
    type Point = F32x2;
    type RadiusSize = f32;
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
    fn get_neigbors(
        &self,
        radius: Self::RadiusSize,
        point: Self::Point,
    ) -> impl Iterator<Item = usize> {
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
        //self.spacial_lookup.iter().enumerate().map(|u| u.0)
    }
}
