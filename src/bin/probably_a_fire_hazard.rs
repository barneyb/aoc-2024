use aoc::aocd;
use aoc::viz::viz_file_name;
use aoc::y2015::probably_a_fire_hazard_06::part_two_array;
use nannou::prelude::*;

const ROWS: u32 = 1000;
const COLS: u32 = 1000;
const SIZE: u32 = 1;
const MARGIN: u32 = 20;
const WIDTH: u32 = COLS * SIZE + 2 * MARGIN;
const HEIGHT: u32 = ROWS * SIZE + 2 * MARGIN;

fn main() {
    nannou::app(model).loop_mode(LoopMode::wait()).run();
}

struct Model {
    line: u32,
    prev: u32,
    array: Vec<u32>,
    max: u32,
}

fn model(app: &App) -> Model {
    app.new_window()
        .title(app.exe_name().unwrap())
        .size(WIDTH, HEIGHT)
        .view(view)
        .key_pressed(key_pressed)
        .mouse_wheel(mouse_wheel)
        .build()
        .unwrap();
    let array = part_two_array(&aocd::get_input(2015, 6).expect("Should have loaded input"));
    Model {
        line: ROWS - 1,
        prev: 0,
        max: *(array.iter().max().unwrap()),
        array,
    }
}

fn key_pressed(app: &App, _: &mut Model, key: Key) {
    match key {
        Key::S => {
            app.main_window().capture_frame(viz_file_name("png"));
        }
        _other_key => {}
    }
}

fn mouse_wheel(_app: &App, model: &mut Model, delta: MouseScrollDelta, _phase: TouchPhase) {
    if let MouseScrollDelta::PixelDelta(pp) = delta {
        model.prev = model.line;
        model.line = if pp.y < 0.0 {
            let dy = pp.y.abs() as u32;
            if dy >= model.line {
                0
            } else {
                model.line - dy
            }
        } else {
            (model.line + pp.y as u32).min(ROWS - 1)
        };
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    // This whole thing's sorta confused between how many lines are showing and
    // which is the last line shown.
    let line = model.line;
    let prev = model.prev;
    let draw = app.draw();
    // let m = MARGIN as f32;
    // let r = Rect::from_w_h(m * 3.0, m).top_left_of(app.window_rect());
    // draw.rect().color(WHITE).xy(r.xy()).wh(r.wh());
    // draw.text(&line.to_string()).color(BLACK).xy(r.xy());
    let draw = draw
        .scale_y(-1.0) // y increases downward
        .x_y(COLS as f32 / -2.0, ROWS as f32 / -2.0) // move the origin to top-left
        ;
    if prev > line {
        let h = (ROWS - line - 1) as f32;
        let w = COLS as f32;
        draw.y(line as f32 + 1.0)
            .rect()
            .color(BLACK)
            .x_y(w / 2.0, h / 2.0)
            .w_h(w, h);
    } else {
        let m = model.max as f32;
        for y in prev..line {
            let dy = (y * COLS) as usize;
            let ydraw = draw.y(y as f32);
            for x in 0..COLS as usize {
                let v = model.array[dy + x];
                if v > 0 {
                    ydraw
                        .rect()
                        .color(gray(v as f32 / m))
                        .x(x as f32)
                        .w_h(1.0, 1.0);
                }
            }
        }
    }
    draw.to_frame(app, &frame).unwrap();
}
