use nannou::prelude::*;

const PX_BETWEEN_VECS: f32 = 10.0;

fn main() {
    nannou::app(Model::new)
        .update(update)
        .simple_window(view)
        .run();
}

#[derive(Debug)]
struct Model {
    field: Vec<Vec2>,
    stride: usize,
}

impl Model {
    fn new(app: &App) -> Model {
        let mut model = Model {
            field: Vec::new(),
            stride: 0,
        };
        model.resize(app);
        model
    }

    fn resize(&mut self, app: &App) {
        let (width, height) = app.main_window().inner_size_pixels();
        let size = vec2(width as f32, height as f32);
        let size = size / PX_BETWEEN_VECS;

        self.stride = size.x as usize;
        self.field
            .resize((size.x * size.y) as usize, vec2(0.0, 0.0))
    }

    fn update(&mut self, mut op: impl FnMut(Vec2) -> Vec2) {
        self.for_each_mut(|pos, vec| *vec = op(pos));
    }

    fn for_each_mut(&mut self, mut op: impl FnMut(Vec2, &mut Vec2)) {
        let stride = self.stride;
        for (i, vec) in self.field.iter_mut().enumerate() {
            let pos = draw_pos(i, stride);
            op(pos, vec);
        }
    }

    fn for_each(&self, mut op: impl FnMut(Vec2, &Vec2)) {
        let stride = self.stride;
        for (i, vec) in self.field.iter().enumerate() {
            let pos = draw_pos(i, stride);
            op(pos, vec);
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

    let (width, height) = app.main_window().inner_size_pixels();
    let size = vec2(width as f32, height as f32);

    model.for_each(|pos, vec| {
        // pos thinks (0, 0) is a corner
        // but nannou thinks (0, 0) is the center
        // so let's translate it appropiately
        let pos = pos - size / 2.0;
        draw.line().start(pos).end(pos + *vec).color(WHITE);
    });

    draw.to_frame(app, &frame).unwrap();
}

fn update(app: &App, model: &mut Model, _update: Update) {
    // TODO: update model using Model::for_each_vec
    model.resize(app);
    model.update(|_| vec2(PX_BETWEEN_VECS, PX_BETWEEN_VECS));
}
