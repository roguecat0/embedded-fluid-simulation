use fluid_simulator::PARTICLES;
use fluid_simulator::math::{Vector, VectorExt};
use fluid_simulator::micromath::vector::F32x2;
use fluid_simulator::neighbor_search::{NeighborSearcher, ParticleSearchStaticMatrix};
use nannou::draw::primitive::Rect;
use nannou::draw::properties::{SetDimensions, SetPosition};
use nannou::prelude::*;
use rand::rngs::ThreadRng;
use rand::{self, Rng};
use std::time::{Duration, Instant};

const CELL_SIZE: f32 = 75.0;
const PARTS_PER_CELL: usize = 10;
const GRID_WIDTH: usize = 5;
const GRID_HEIGHT: usize = 5;
const GRID_SIZE: usize = GRID_WIDTH * GRID_HEIGHT;
const WIDTH: f32 = GRID_WIDTH as f32 * CELL_SIZE;
const HEIGTH: f32 = GRID_HEIGHT as f32 * CELL_SIZE;

struct Model {
    last_frame: Instant,
    neighbor_searcher: ParticleSearchStaticMatrix<GRID_SIZE, PARTS_PER_CELL>,
    particles: [F32x2; PARTICLES],
}

fn main() {
    nannou::app(model).update(update).run();
}
fn update(_app: &App, model: &mut Model, _update: Update) {
    println!("---- start ----");
    model.neighbor_searcher.build(&model.particles);
}
fn vec_to_point(vec2: Vec2) -> F32x2 {
    F32x2 {
        x: vec2.x,
        y: vec2.y,
    }
}

trait FromCorner {
    fn from_corner(self, bl: Vec2, size: Vec2) -> Self;
}
impl FromCorner for nannou::draw::Drawing<'_, Rect> {
    // add code here
    fn from_corner(self, bl: Vec2, size: Vec2) -> Self {
        let center = bl + (size / 2.0);
        self.xy(center).wh(size)
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let mut draw = app.draw();
    draw.background().color(BLACK);

    let rect = app.window_rect();

    let offset = (-rect.w() * 0.5, -rect.h() * 0.5).into();
    draw = draw.xy(offset);
    let m_pos = app.mouse.position() - offset;

    dbg!(&model.neighbor_searcher.cells);
    dbg!(&model.particles);
    // box
    draw.rect()
        .color(BLACK)
        .stroke(BLUE)
        .stroke_weight(3.0)
        .from_corner((0.0, 0.0).into(), (WIDTH, HEIGTH).into());
    // grid_cells
    for x in 0..GRID_WIDTH {
        for y in 0..GRID_HEIGHT {
            draw.rect()
                .color(BLACK)
                .stroke(YELLOW)
                .stroke_weight(1.5)
                .from_corner(
                    (x as f32 * CELL_SIZE, y as f32 * CELL_SIZE).into(),
                    (CELL_SIZE, CELL_SIZE).into(),
                );
        }
    }
    if m_pos.x < WIDTH && m_pos.y < HEIGTH {
        draw.ellipse().color(RED).radius(3.0).xy(m_pos);
        for (x, y) in model
            .neighbor_searcher
            .gen_valid_cell_indexes(vec_to_point(m_pos))
        {
            println!("point: {:?}", (x, y));
            draw.rect()
                .color(BLACK)
                .stroke(RED)
                .stroke_weight(1.5)
                .from_corner(
                    (x as f32 * CELL_SIZE, y as f32 * CELL_SIZE).into(),
                    (CELL_SIZE, CELL_SIZE).into(),
                );
        }
        for point_i in model.neighbor_searcher.get_neigbors(vec_to_point(m_pos)) {
            let point = model.particles[point_i].to_array().into();
            draw.ellipse().color(ORANGE).radius(5.0).xy(point);
        }
    }
    // paricles
    for particle in model.particles {
        let p: Vec2 = particle.to_array().into();
        draw.ellipse().color(WHITE).radius(3.0).xy(p);
    }
    draw.to_frame(app, &frame).unwrap();
}
fn rand_vector2d(rng: &mut ThreadRng) -> F32x2 {
    F32x2::from_slice(&[rng.random_range(0.0..WIDTH), rng.random_range(0.0..HEIGTH)])
}

fn model(app: &App) -> Model {
    let mut rng = rand::rng();
    let _window = app.new_window().size(512, 512).view(view).build().unwrap();
    let particles: [F32x2; PARTICLES] = std::array::from_fn(|_| rand_vector2d(&mut rng));
    Model {
        neighbor_searcher: ParticleSearchStaticMatrix::new(GRID_WIDTH, CELL_SIZE),
        last_frame: Instant::now(),
        particles,
    }
}
