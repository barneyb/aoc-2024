use aoc::aocd;
use aoc::y2015::probably_a_fire_hazard_06::part_one_array;
use nannou::color::{BLACK, WHITE};
use nannou::prelude::Update;
use nannou::{App, Frame, LoopMode};

const ROWS: u32 = 1000;
const COLS: u32 = 1000;
const SIZE: u32 = 1;
const MARGIN: u32 = 10;
const WIDTH: u32 = COLS * SIZE + 2 * MARGIN;
const HEIGHT: u32 = ROWS * SIZE + 2 * MARGIN;

fn main() {
    nannou::app(model)
        .update(update)
        .loop_mode(LoopMode::loop_once())
        .run();
}

struct Model {
    array: Vec<bool>,
}

fn model(app: &App) -> Model {
    app.new_window()
        .title(app.exe_name().unwrap())
        .size(WIDTH, HEIGHT)
        .view(view)
        .build()
        .unwrap();
    Model {
        array: part_one_array(&aocd::get_input(2015, 6).expect("Should have loaded input")),
    }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, model: &Model, frame: Frame) {
    frame.clear(BLACK);
    let draw = app.draw()
        .scale_y(-1.0) // y increases downward
        .x_y(COLS as f32 / -2.0, ROWS as f32 / -2.0) // move the origin to top-left
        ;
    for y in 0..COLS as usize {
        let dy = y * 1000;
        let ydraw = draw.y(y as f32);
        for x in 0..ROWS as usize {
            if model.array[dy + x] {
                ydraw.rect().color(WHITE).x_y(x as f32, 0.0).w_h(1.0, 1.0);
            }
        }
    }
    draw.to_frame(app, &frame).unwrap();
}
