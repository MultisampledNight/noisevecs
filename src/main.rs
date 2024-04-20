use std::cell::Cell;

use nannou::prelude::*;
use noise::{NoiseFn, Simplex};

const PX_BETWEEN_VECS: f32 = 20.0;
const VEC_LEN: f32 = PX_BETWEEN_VECS * 2.0;
const NOISE_SCALE: f32 = 0.002;
const TIME_SCALE: f32 = 0.2;

fn main() {
    nannou::app(Model::new)
        .update(update)
        .simple_window(view)
        .run();
}

#[derive(Debug)]
struct Model {
    field: Vec<(Vec2, Rgb)>,
    stride: usize,
    generator: Cell<Simplex>,
}

impl Model {
    fn new(app: &App) -> Model {
        let mut model = Model {
            field: Vec::new(),
            stride: 0,
            generator: Cell::new(Simplex::new(1)),
        };
        model.resize(app);
        model
    }

    fn resize(&mut self, app: &App) {
        let size = size(app);
        let size = size / PX_BETWEEN_VECS;

        self.stride = size.x as usize;
        self.field.resize(
            (size.x * size.y) as usize,
            (vec2(0.0, 0.0), rgb(0.0, 0.0, 0.0)),
        )
    }

    fn update(&mut self, mut op: impl FnMut(Vec2) -> (Vec2, Rgb)) {
        self.for_each_mut(|pos, vec| *vec = op(pos));
    }

    fn for_each_mut(&mut self, mut op: impl FnMut(Vec2, &mut (Vec2, Rgb))) {
        let stride = self.stride;
        for (i, cell) in self.field.iter_mut().enumerate() {
            let pos = draw_pos(i, stride);
            op(pos, cell);
        }
    }

    fn for_each(&self, mut op: impl FnMut(Vec2, &(Vec2, Rgb))) {
        let stride = self.stride;
        for (i, cell) in self.field.iter().enumerate() {
            let pos = draw_pos(i, stride);
            op(pos, cell);
        }
    }
}

fn draw_pos(i: usize, stride: usize) -> Vec2 {
    let x = i % stride;
    let y = i / stride;
    vec2(x as f32 * PX_BETWEEN_VECS, y as f32 * PX_BETWEEN_VECS)
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);

    let size = size(app);

    model.for_each(|pos, (vec, color)| {
        // pos thinks (0, 0) is a corner
        // but nannou thinks (0, 0) is the center
        // so let's translate it appropiately
        let pos = pos - size / 2.0;

        let lightness = vec.length().sqrt();
        let color = rgb(
            color.red * lightness,
            color.green * lightness,
            color.blue * lightness,
        );

        let vec = *vec * VEC_LEN;

        draw.line()
            .start(pos)
            .end(pos + vec)
            .color(color)
            .weight(4.0)
            .caps_round();
    });

    draw.to_frame(app, &frame).unwrap();
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let gen = model.generator.take();

    model.resize(app);
    model.update(|pos| noise(gen, pos, app.time));

    model.generator.set(gen);
}

fn size(app: &App) -> Vec2 {
    let (width, height) = app.main_window().inner_size_pixels();
    vec2(width as f32, height as f32)
}

fn noise(gen: Simplex, mut pos: Vec2, time: f32) -> (Vec2, Rgb) {
    pos *= NOISE_SCALE;
    let scalar = |axis| {
        gen.get([
            pos.x as f64,
            pos.y as f64,
            (time * TIME_SCALE) as f64,
            axis as f64 * 3000.0,
        ]) as f32
    };
    (
        vec2(scalar(0), scalar(1)),
        rgb(scalar(2), scalar(3), scalar(4)),
    )
}
