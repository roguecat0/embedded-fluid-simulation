#![no_std]

pub fn hello_world() -> &'static str {
    "hello me"
}
pub mod kernel;
pub mod math;
pub mod neighbor_search;
use core::mem;
use kernel::{Kernel, StdKernel2};
use math::{F32x2, VectorExt};
pub use micromath;
use micromath::{F32Ext, vector::Vector};
use neighbor_search::{NeighborSearcher, ParticleSearchStaticMatrix};

// 64 * 3 = 192 bit
// not doing wind ...
pub const PARTICLES: usize = 2;
const GRAVITY: F32x2 = F32x2 { x: 0.0, y: -9.81 };
const MASS: f32 = 0.1;

const GRID_WIDTH: usize = 5;
const GRID_HEIGHT: usize = 5;
const GRID_SIZE: usize = GRID_WIDTH * GRID_HEIGHT;
const PARTS_PER_CELL: usize = 10;
const RADIUS: f32 = 5.0;
const KERNAL: StdKernel2 = StdKernel2::new(RADIUS);

pub struct ParticleSystemData2d {
    pub positions: [F32x2; PARTICLES],
    pub velocities: [F32x2; PARTICLES],
    pub forces: [F32x2; PARTICLES],
    densities: [f32; PARTICLES],
}
impl ParticleSystemData2d {
    pub fn gradient_at(
        &self,
        i: usize,
        values: &[f32],
        neighbors: impl Iterator<Item = usize>,
    ) -> F32x2 {
        let mut sum = F32x2::default();
        let origin = self.positions[i];

        for (j, neighbor) in neighbors.map(|j| (j, self.positions[i])) {
            let distance = origin.distance(neighbor);
            if distance > 0.0 {
                let dir = (neighbor - origin).div(distance);
                sum += KERNAL.gradient(distance, dir)
                    * (values[j] * MASS)
                    * (values[i] / self.densities[i].powi(2)
                        + values[j] / self.densities[j].powi(2));
            }
        }
        sum
    }
    pub fn laplacian_at(
        &self,
        i: usize,
        values: &[f32],
        neighbors: impl Iterator<Item = usize>,
    ) -> f32 {
        let mut sum = f32::default();
        let origin = self.positions[i];

        for (j, neighbor) in neighbors.map(|j| (j, self.positions[i])) {
            let distance = origin.distance(neighbor);
            if distance > 0.0 {
                sum += MASS * (values[j] - values[i]) / self.densities[j]
                    * KERNAL.second_derivitive(distance);
            }
        }
        sum
    }
}
pub struct BoxBorders {
    top: f32,
    bottom: f32,
    left: f32,
    right: f32,
}

pub struct ParticleSystemSolver2d {
    data: ParticleSystemData2d,
    neighbor_searcher: ParticleSearchStaticMatrix<GRID_SIZE, PARTS_PER_CELL>,
    // new posistion
    new_positions: [F32x2; PARTICLES],
    new_velocities: [F32x2; PARTICLES],
    box_borders: BoxBorders,
}
impl ParticleSystemSolver2d {
    pub fn advance_timestep(&mut self, delta: f32) {
        self.begin_timestep();
        self.accumelate_forces(delta);
        self.time_integration(delta);
        self.resolve_collisions();
        self.end_timestep();
    }
    #[inline]
    fn begin_timestep(&mut self) {
        // reset forces but that is waistfull if it it done by gravity anyway
    }
    #[inline]
    fn end_timestep(&mut self) {
        // new positions is dirty but just override it before using it
        mem::swap(&mut self.new_velocities, &mut self.data.velocities);
        mem::swap(&mut self.new_positions, &mut self.data.positions);
    }
    #[inline]
    fn accumelate_forces(&mut self, delta: f32) {
        self.accumelate_external_forces();
    }
    #[inline]
    fn accumelate_external_forces(&mut self) {
        for i in 0..PARTICLES {
            self.data.forces[i] = GRAVITY * MASS;
        }
    }
    #[inline]
    fn time_integration(&mut self, delta: f32) {
        for i in 0..PARTICLES {
            self.new_velocities[i] =
                self.data.velocities[i] + (self.data.forces[i].div(delta / MASS));
            self.new_positions[i] = self.new_velocities[i] * delta + self.data.positions[i];
        }
    }
    #[inline]
    fn resolve_collisions(&mut self) {
        // todo: p107
    }
    pub fn interpolate(&self, point: F32x2, values: &[F32x2]) -> F32x2 {
        let mut sum = F32x2::default();
        for (i, pos) in self
            .neighbor_searcher
            .get_neigbors(point)
            .map(|i| (i, self.new_positions[i]))
        {
            let distance = point.distance(pos);
            let weight = MASS / self.data.densities[i] * KERNAL.eval(distance);
            sum += values[i] * weight;
        }
        sum
    }
    fn update_densities(&mut self) {
        for (i, pos) in self.data.positions.iter().enumerate() {
            let sum = self.kernal_sum(*pos);
            self.data.densities[i] = sum * MASS;
        }
    }
    fn kernal_sum(&self, point: F32x2) -> f32 {
        let mut sum = 0f32;
        for pos in self
            .neighbor_searcher
            .get_neigbors(point)
            .map(|i| self.data.positions[i])
        {
            let distance = point.distance(pos);
            sum += KERNAL.eval(distance);
        }
        sum
    }
}
