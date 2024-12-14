use aoc::viz::viz_file_name;
use nannou::color::{Alpha, Hue};
use nannou::prelude::*;
use std::sync::Mutex;

const MARGIN: usize = 20;
const HEIGHT: usize = 1020;

fn main() {
    // aoc::with_input(2024, 12, do_solve)
    nannou::app(model)
        .update(update)
        .loop_mode(LoopMode::wait())
        .run();
}

struct Growth {
    stage_count: u8,
    lo: f32,
    hi: f32,
    color: Alpha<Srgb<u8>, f32>,
}

impl Growth {
    pub(crate) fn jitter(&self, scale: f32) -> Point2 {
        pt2(
            scale * random_range(self.lo, self.hi),
            scale * random_range(self.lo, self.hi),
        )
    }
}

impl Growth {
    fn new(color: Srgb<u8>, shift: f32, stage_count: u8, jitter: f32) -> Growth {
        // this is just STUPID...
        let Srgb {
            red, green, blue, ..
        } = color;
        let red = red as f32;
        let green = green as f32;
        let blue = blue as f32;
        let hsl = Hsl::from(Rgb::new(red, green, blue)).shift_hue(random_range(-shift, shift));
        let Srgb {
            red, green, blue, ..
        } = Rgb::from(hsl);
        let red = red as u8;
        let green = green as u8;
        let blue = blue as u8;
        Growth {
            stage_count,
            lo: random_range(-1.0 * jitter, 0.0),
            hi: random_range(0.0, jitter),
            color: Alpha {
                color: Rgb::new(red, green, blue),
                alpha: 0.6,
            },
        }
    }
}

struct Plant {
    label: String,
    sprout: Growth,
    flower: Growth,
}

struct Model {
    offset: f32,
    scale: f32,
    font_size: u32,
    fence_weight: f32,
    grid: Vec<Vec<usize>>,
    plants: Vec<Plant>,
}

fn model(app: &App) -> Model {
    app.new_window()
        .title(app.exe_name().unwrap())
        .size(HEIGHT as u32, HEIGHT as u32)
        .view(view)
        .key_pressed(key_pressed)
        .build()
        .unwrap();
    let input = aoc::aocd::get_input(2024, 12).unwrap();
    let grid: Vec<Vec<_>> = input
        .lines()
        .map(|l| l.chars().map(|c| c as usize - 'A' as usize).collect())
        .collect();
    let scale = ((HEIGHT - MARGIN * 2) / grid.len()) as f32;
    let plants = ('A'..='Z')
        .map(|c| Plant {
            label: format!("{c}"),
            sprout: Growth::new(GREEN, 30.0, random_range(4, 8), 0.2),
            flower: Growth::new(RED, 60.0, random_range(6, 12), 0.5),
        })
        .collect();
    Model {
        offset: -1.0 * (HEIGHT / 2 - MARGIN) as f32,
        scale,
        font_size: scale as u32,
        fence_weight: 1.0.max(scale / 10.0),
        grid,
        plants,
    }
}

fn key_pressed(app: &App, _model: &mut Model, key: Key) {
    match key {
        Key::Q => app.quit(),
        Key::S => {
            app.main_window().capture_frame(viz_file_name("png"));
        }
        _other_key => {}
    }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, model: &Model, frame: Frame) {
    static VIEW_COUNTER: Mutex<u32> = Mutex::new(0);
    let counter = {
        let mut counter = VIEW_COUNTER.lock().unwrap();
        *counter += 1;
        *counter
    };
    println!("view {counter}");
    if counter > 1 {
        // the render never changes...
        return;
    }

    // Colors
    let c_field = BLACK;
    let c_fence = SADDLEBROWN;
    let c_stubble = PERU;

    // Aliases, for shorter expressions
    let s = model.scale;

    // Initialize
    let draw = app.draw().x_y(model.offset, model.offset);
    draw.background().color(c_field);

    // Fences first
    let bound = pt2(model.grid[0].len() as f32 * s, model.grid.len() as f32 * s);
    for st in [pt2(0.0, 0.0), bound] {
        draw.line()
            .start(st)
            .end(pt2(0.0, bound.y))
            .weight(model.fence_weight)
            .color(c_fence);
        draw.line()
            .start(st)
            .end(pt2(bound.x, 0.0))
            .weight(model.fence_weight)
            .color(c_fence);
    }
    for (y, line) in model.grid.iter().enumerate() {
        for (x, &c) in line.iter().enumerate() {
            let start = pt2(x as f32 * s, y as f32 * s);
            if x > 0 && line[x - 1] != c {
                draw.line()
                    .start(start)
                    .end(pt2(start.x, start.y + s))
                    .weight(model.fence_weight)
                    .color(c_fence);
            }
            if y > 0 && model.grid[y - 1][x] != c {
                draw.line()
                    .start(start)
                    .end(pt2(start.x + s, start.y))
                    .weight(model.fence_weight)
                    .color(c_fence);
            }
        }
    }

    // Finally, the plants!
    for (y, line) in model.grid.iter().enumerate() {
        for (x, &c) in line.iter().enumerate() {
            let plant = &model.plants[c];
            let start = pt2(x as f32 * s + s / 2.0, y as f32 * s + s / 1.5);
            draw.text(&plant.label)
                .xy(start)
                .font_size(model.font_size)
                .color(c_stubble);
        }
    }
    draw_growth(model, &draw, |plant| &plant.sprout);
    draw_growth(model, &draw, |plant| &plant.flower);

    // Send it!
    draw.to_frame(app, &frame).unwrap();
}

fn draw_growth<F>(model: &Model, draw: &Draw, get_growth: F)
where
    F: Fn(&Plant) -> &Growth,
{
    let s = model.scale;
    for (y, line) in model.grid.iter().enumerate() {
        for (x, &c) in line.iter().enumerate() {
            let plant = &model.plants[c];
            let growth = get_growth(plant);
            let start = pt2(x as f32 * s + s / 2.0, y as f32 * s + s / 1.5);
            for _ in 0..growth.stage_count {
                let jitter = growth.jitter(s);
                draw.text(&plant.label)
                    .xy(start + jitter)
                    .font_size(model.font_size)
                    .color(growth.color);
            }
        }
    }
}
