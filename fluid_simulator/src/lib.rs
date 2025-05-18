#![no_std]

pub fn hello_world() -> &'static str {
    "hello me"
}
pub mod math;
pub mod neighbor_search;
use core::mem;
use math::{F32x2, VectorExt};
pub use micromath;

// 64 * 3 = 192 bit
// not doing wind ...
const PARTICLES: usize = 40;
const GRAVITY: F32x2 = F32x2 { x: 0.0, y: -9.81 };
const MASS: f32 = 0.1;

pub struct ParticleSystemData2d {
    pub positions: [F32x2; PARTICLES],
    pub velocities: [F32x2; PARTICLES],
    pub forces: [F32x2; PARTICLES],
}
pub struct BoxBorders {
    top: f32,
    bottom: f32,
    left: f32,
    right: f32,
}

pub struct ParticleSystemSolver2d {
    data: ParticleSystemData2d,
    // new posistion
    new_positions: [F32x2; PARTICLES],
    new_velocities: [F32x2; PARTICLES],
    box_borders: BoxBorders,
}
pub struct NeigborSearcher {}
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
}
