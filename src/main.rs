use nannou::{color::Lch, prelude::*};
use noise::{NoiseFn, Simplex};
use rand::prelude::*;

const PX_BETWEEN_VECS: f32 = 20.0;
const VEC_LEN: f32 = PX_BETWEEN_VECS * 3.5;
const NOISE_SCALE: f32 = 0.002;
const TIME_SCALE: f32 = 0.15;
const SQUIRLINESS: f32 = 2.0;
const CHAOTICNESS: f32 = 0.2;

fn main() {
    nannou::app(Model::new)
        .update(update)
        .simple_window(view)
        .run();
}

#[derive(Debug)]
struct Model {
    field: Vec<Cell>,
    stride: usize,
    generator: std::cell::Cell<Simplex>,
}

#[derive(Clone, Copy, Debug, Default)]
struct Cell {
    value: Vec2,
    chroma: f32,
    hue: f32,
}

impl Model {
    fn new(app: &App) -> Model {
        let mut rng = thread_rng();
        let mut model = Model {
            field: Vec::new(),
            stride: 0,
            generator: std::cell::Cell::new(Simplex::new(rng.gen())),
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
            Cell::default(),
        )
    }

    fn update(&mut self, mut op: impl FnMut(Vec2) -> Cell) {
        self.for_each_mut(|pos, cell| *cell = op(pos));
    }

    fn for_each_mut(&mut self, mut op: impl FnMut(Vec2, &mut Cell)) {
        let stride = self.stride;
        for (i, cell) in self.field.iter_mut().enumerate() {
            let pos = draw_pos(i, stride);
            op(pos, cell);
        }
    }

    fn for_each(&self, mut op: impl FnMut(Vec2, &Cell)) {
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

    model.for_each(|pos, Cell { value, chroma, hue }| {
        // pos thinks (0, 0) is a corner
        // but nannou thinks (0, 0) is the center
        // so let's translate it appropiately
        let pos = pos - size / 2.0;

        let lightness = value.length().sqrt();
        let color = Lch::new(
            30.0 + lightness * 70.0,
            20.0 + *chroma * 40.0,
            *hue * 360.0,
        );

        let value = *value * VEC_LEN;

        draw.line()
            .start(pos)
            .end(pos + value)
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

fn noise(gen: Simplex, mut pos: Vec2, time: f32) -> Cell {
    pos *= NOISE_SCALE;
    let scalar = |axis, scale| {
        gen.get([
            pos.x as f64 * scale,
            pos.y as f64 * scale,
            (time * TIME_SCALE) as f64,
            axis as f64 * 100_000.0,
        ]) as f32
    };

    let magnitude = scalar(0, 1.0);
    let rotation = scalar(1, 1.0);
    let chaos = scalar(2, 100.0);

    let mut value = vec2(magnitude + (chaos - 0.5) * 2.0 * CHAOTICNESS, 0.0);

    let pi = std::f32::consts::PI;
    value = value.rotate(rotation * pi * SQUIRLINESS);

    let chroma = scalar(3, 1.0);
    let hue = scalar(4, 1.0);

    Cell {
        value,
        chroma,
        hue,
    }
}
