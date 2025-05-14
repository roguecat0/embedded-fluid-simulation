/*
 * Simple Loop example with 4D noise
 * Daily Sketch 2019/09/23 by Alexis Andre (@mactuitui)
 *
 * Demonstration of looping an animation using periodic functions.
 *
 */

use nannou::prelude::*;
use std::time::{Duration, Instant};

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    radius: f32,
    last_frame: Instant,
}

fn model(app: &App) -> Model {
    let _window = app
        .new_window()
        .size(1024, 1024)
        .view(view)
        .build()
        .unwrap();
    Model {
        radius: 1f32,
        last_frame: Instant::now(),
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    // Prepare to draw.
    let draw = app.draw();

    draw.background().color(BLACK);

    draw.ellipse()
        .x_y(0f32, 0f32)
        .radius(model.radius)
        .color(RED);

    draw.to_frame(app, &frame).unwrap();
}

fn update(app: &App, model: &mut Model, update: Update) {
    let elapsed_time = Instant::now() - model.last_frame;
    let delta = elapsed_time.secs();

    model.radius += 5f32 * delta as f32;
    model.radius = model.radius % 100f32;
    model.last_frame = Instant::now();
    println!("{}", model.radius);
}
