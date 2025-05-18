/*
 * Simple Loop example with 4D noise
 * Daily Sketch 2019/09/23 by Alexis Andre (@mactuitui)
 *
 * Demonstration of looping an animation using periodic functions.
 *
 */

use fluid_simulator::math::VectorExt;
use fluid_simulator::micromath::vector::{F32x2, Vector};
use nannou::prelude::*;
use std::{
    f32,
    time::{Duration, Instant},
};

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    last_frame: Instant,
    anim: SpringAnimation,
}

const POINTS: usize = 5;
const MASS: f32 = 10.0;
const GRAVITY: F32x2 = F32x2 { x: 0.0, y: -9.81 };
const STIFFNESS: f32 = 200f32;
const REST_LENGTH: f32 = 1.2;
const DAMPENING_COEF: f32 = 5.0;
const AIRDRAG_COEF: f32 = 1.0;
const TOPY: f32 = 5f32;
const FLOOR_Y: f32 = TOPY - 8.0;
const RESTITUTION_COEF: f32 = 0.7;
const UPSCALE: f32 = 50f32;

struct Constraint {
    point_index: usize,
    position: F32x2,
}

struct SpringAnimation {
    positions: [F32x2; POINTS],
    velocities: [F32x2; POINTS],
    forces: [F32x2; POINTS],
    edges: [(usize, usize); POINTS - 1],
    constraints: [Constraint; 1],
}
impl SpringAnimation {
    pub fn new() -> Self {
        let mut positions = [F32x2 { x: 0.0, y: TOPY }; POINTS];
        for (i, position) in positions.iter_mut().enumerate() {
            position.x = i as f32 * 1.2;
        }
        let mut edges = [(0, 0); POINTS - 1];
        for (i, edge) in edges.iter_mut().enumerate() {
            *edge = (i, i + 1);
        }

        Self {
            positions,
            velocities: [F32x2 { x: 0.0, y: 0.0 }; POINTS],
            forces: [F32x2 { x: 0.0, y: 0.0 }; POINTS],
            constraints: [Constraint {
                point_index: 0,
                position: F32x2 { x: 0.0, y: TOPY },
            }],
            edges,
        }
    }
    pub fn advance_step(&mut self, delta: f32) {
        // calc forces
        for i in 0..POINTS {
            // gravity force
            self.forces[i] = GRAVITY * MASS;
            // airdrag force
            self.forces[i] += self.velocities[i] * (-AIRDRAG_COEF);
        }
        println!(" --- START UPDATE --- delta: {delta}");
        for i in 0..(POINTS - 1) {
            let i0 = self.edges[i].0;
            let i1 = self.edges[i].1;
            let pos0 = self.positions[i0];
            let pos1 = self.positions[i1];
            let r = pos0 - pos1;
            let distance = r.magnitude();
            // spring force
            if distance > 0f32 {
                let force = r.div(distance) * (-STIFFNESS * (distance - REST_LENGTH));
                self.forces[i0] += force;
                self.forces[i1] -= force;
            }
            // dampening force
            let vel0 = self.velocities[i0];
            let vel1 = self.velocities[i1];
            let dampening = (vel0 - vel1) * -DAMPENING_COEF;
            self.forces[i0] += dampening;
            self.forces[i1] -= dampening;
        }
        // update states
        for i in 0..POINTS {
            // compute states
            let acceleration = self.forces[i].div(MASS);
            let mut tmp_vel = self.velocities[i] + acceleration * delta;
            let mut tmp_pos = self.positions[i] + tmp_vel * delta;
            // colisions
            if tmp_pos.y < FLOOR_Y {
                tmp_vel.y *= -RESTITUTION_COEF;
                tmp_pos.y = FLOOR_Y;
                tmp_pos += tmp_vel * delta;
                println!("collision -------- ");
            }
            // update state
            self.velocities[i] = tmp_vel;
            self.positions[i] = tmp_pos;
            println!("acc: {acceleration:?}, vel: {tmp_vel:?}, pos: {tmp_pos:?}");
        }
        // apply constraints
        for constraint in self.constraints.iter() {
            self.positions[constraint.point_index] = constraint.position;
            self.velocities[constraint.point_index] = F32x2 { x: 0f32, y: 0f32 };
        }
    }
}

fn model(app: &App) -> Model {
    let _window = app
        .new_window()
        .size(1024, 1024)
        .view(view)
        .build()
        .unwrap();
    Model {
        last_frame: Instant::now(),
        anim: SpringAnimation::new(),
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    // Prepare to draw.
    let draw = app.draw();
    let rect = app.window_rect();

    draw.background().color(BLACK);

    draw.line()
        .weight(10.0)
        .caps_round()
        .points(
            (rect.left(), FLOOR_Y * UPSCALE).into(),
            (rect.right(), FLOOR_Y * UPSCALE).into(),
        )
        .color(WHITE);
    for edge in model.anim.edges.iter() {
        let p0 = model.anim.positions[edge.0];
        let p1 = model.anim.positions[edge.1];
        draw.line().weight(10f32).color(BLUE).caps_round().points(
            (p0.x * UPSCALE, p0.y * UPSCALE).into(),
            (p1.x * UPSCALE, p1.y * UPSCALE).into(),
        );
    }
    for point in model.anim.positions.iter() {
        draw.ellipse()
            .x_y(point.x * UPSCALE, point.y * UPSCALE)
            .color(RED)
            .radius(8.0);
    }

    draw.to_frame(app, &frame).unwrap();
}

fn update(app: &App, model: &mut Model, update: Update) {
    let elapsed_time = Instant::now() - model.last_frame;
    let delta = elapsed_time.secs() as f32;
    if delta > 1f32 / 60f32 {
        model.last_frame = Instant::now();
        model.anim.advance_step(delta);
    }
}
