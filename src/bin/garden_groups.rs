use aoc::aocd;
use aoc::viz::viz_file_name;
use nannou::prelude::*;

const ROWS: u32 = 140;
const COLS: u32 = 140;
const SIZE: u32 = 7;
const MARGIN: u32 = 20;
const WIDTH: u32 = COLS * SIZE + 2 * MARGIN;
const HEIGHT: u32 = ROWS * SIZE + 2 * MARGIN;

fn main() {
    // aoc::with_input(2024, 12, do_solve)
    nannou::app(model)
        .update(update)
        .loop_mode(LoopMode::wait())
        .run();
}

struct Model {}

fn model(app: &App) -> Model {
    app.new_window()
        .title(app.exe_name().unwrap())
        .size(WIDTH, HEIGHT)
        .view(view)
        .key_pressed(key_pressed)
        .build()
        .unwrap();
    aocd::get_input(2024, 12);
    Model {}
}

fn key_pressed(app: &App, _model: &mut Model, key: Key) {
    match key {
        Key::S => {
            app.main_window().capture_frame(viz_file_name("png"));
        }
        _other_key => {}
    }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, model: &Model, frame: Frame) {}
