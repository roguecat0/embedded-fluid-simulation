use fluid_simulator::PARTICLES;
use fluid_simulator::math::VectorExt;
use fluid_simulator::micromath::vector::{F32x2, Vector};
use fluid_simulator::neighbor_search::{NeighborSearcher, ParticleSearchHeapless};
use nannou::prelude::*;
use std::time::{Duration, Instant};

const CEL_SIZE: f32 = 64.0;

struct Model {
    last_frame: Instant,
    neighbor_searcher: ParticleSearchHeapless,
    particles: [F32x2; PARTICLES],
}

fn main() {
    nannou::app(model).update(update).run();
}
fn update(app: &App, model: &mut Model, update: Update) {
    model.neighbor_searcher.build(&model.particles);
}
fn coord_to_point(coord: (isize, isize)) -> Vec2 {
    (coord.0 as f32 * CEL_SIZE, coord.1 as f32 * CEL_SIZE).into()
}
fn vec_to_point(vec2: Vec2) -> F32x2 {
    F32x2 {
        x: vec2.x,
        y: vec2.y,
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let mut draw = app.draw();
    let rect = app.window_rect();
    draw = draw.x_y(-rect.w() * 0.5, -rect.h() * 0.5);
    draw.background().color(BLACK);
    draw.rect()
        .color(BLACK)
        .x_y(CEL_SIZE / 2f32, CEL_SIZE / 2f32)
        .w_h(CEL_SIZE * 5f32, CEL_SIZE * 5f32)
        .stroke_weight(2.0)
        .stroke(BLUE);

    let cz = CEL_SIZE as isize;
    println!(" ------------- START --------------");
    dbg!(&model.particles);
    dbg!(&model.neighbor_searcher.spacial_lookup);
    dbg!(&model.neighbor_searcher.start_index_map);

    for i in ((-cz * 8)..=(cz * 8)).step_by(cz as usize) {
        draw.line()
            .color(WHITE)
            .weight(1.0)
            .points((-512f32, i as f32).into(), (512f32, i as f32).into());

        draw.line()
            .color(WHITE)
            .weight(1.0)
            .points((i as f32, -512f32).into(), (i as f32, 512f32).into());
    }
    let m_pos = app.mouse.position() + Vec2::from_slice(&[rect.w() / 2.0, rect.h() / 2.0]);
    let m_pos = vec_to_point(m_pos);
    //let coord = model.neighbor_searcher.get_grid_index(&m_pos);
    //let pos = coord_to_point((coord.0 as isize, coord.1 as isize))
    //    + Vec2::from_slice(&[CEL_SIZE / 2.0, CEL_SIZE / 2.0]);
    for coord in model.neighbor_searcher.get_cells(m_pos) {
        let pos = coord_to_point((coord.0 as isize, coord.1 as isize))
            + Vec2::from_slice(&[CEL_SIZE / 2.0, CEL_SIZE / 2.0]);
        draw.rect()
            .color(BLACK)
            .xy(pos)
            .w_h(CEL_SIZE, CEL_SIZE)
            .stroke_weight(2.0)
            .stroke(RED);
    }

    for part_i in model.neighbor_searcher.get_neigbors(m_pos) {
        dbg!(&part_i);
        let coord = model
            .neighbor_searcher
            .get_grid_index(&model.particles[part_i]);
        let pos = coord_to_point((coord.0 as isize, coord.1 as isize))
            + Vec2::from_slice(&[CEL_SIZE / 2.0, CEL_SIZE / 2.0]);
        draw.rect()
            .color(BLACK)
            .xy(pos)
            .w_h(CEL_SIZE, CEL_SIZE)
            .stroke_weight(1.0)
            .stroke(YELLOW);
    }

    for particle in model.particles {
        draw.ellipse()
            .radius(10.0)
            .color(GREEN)
            .xy(particle.to_array().into());
    }
    draw.to_frame(app, &frame).unwrap();
}

fn model(app: &App) -> Model {
    let _window = app.new_window().size(512, 512).view(view).build().unwrap();
    Model {
        neighbor_searcher: ParticleSearchHeapless::new(CEL_SIZE),
        particles: [
            F32x2::from_slice(&[CEL_SIZE / 2.0, CEL_SIZE / 2f32]),
            F32x2::from_slice(&[CEL_SIZE + CEL_SIZE / 2.0, CEL_SIZE + CEL_SIZE / 2f32]),
        ],
        last_frame: Instant::now(),
    }
}
