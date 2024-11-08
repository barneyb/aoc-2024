use aoc::aocd;
use aoc::y2015::probably_a_fire_hazard_06::part_one_array;
use nannou::color::{BLACK, WHITE};
use nannou::event::{MouseScrollDelta, TouchPhase};
use nannou::geom::Rect;
use nannou::prelude::Update;
use nannou::{App, Frame, LoopMode};

const ROWS: u32 = 1000;
const COLS: u32 = 1000;
const SIZE: u32 = 1;
const MARGIN: u32 = 20;
const WIDTH: u32 = COLS * SIZE + 2 * MARGIN;
const HEIGHT: u32 = ROWS * SIZE + 2 * MARGIN;

fn main() {
    nannou::app(model)
        .update(update)
        .loop_mode(LoopMode::wait())
        .run();
}

struct Model {
    line: u32,
    prev: u32,
    array: Vec<bool>,
}

fn model(app: &App) -> Model {
    app.new_window()
        .title(app.exe_name().unwrap())
        .size(WIDTH, HEIGHT)
        .view(view)
        .mouse_wheel(mouse_wheel)
        .build()
        .unwrap();
    Model {
        line: 0,
        prev: 0,
        array: part_one_array(&aocd::get_input(2015, 6).expect("Should have loaded input")),
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

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, model: &Model, frame: Frame) {
    // This whole thing's sorta confused between how many lines are showing and
    // which is the last line shown.
    let line = model.line;
    let prev = model.prev;
    let draw = app.draw();
    let m = MARGIN as f32;
    let r = Rect::from_w_h(m * 3.0, m).top_left_of(app.window_rect());
    draw.rect().color(WHITE).xy(r.xy()).wh(r.wh());
    draw.text(&line.to_string()).color(BLACK).xy(r.xy());
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
        for y in prev..line {
            let dy = (y * COLS) as usize;
            let ydraw = draw.y(y as f32);
            for x in 0..COLS as usize {
                if model.array[dy + x] {
                    ydraw.rect().color(WHITE).x(x as f32).w_h(1.0, 1.0);
                }
            }
        }
    }
    draw.to_frame(app, &frame).unwrap();
}
