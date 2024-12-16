use aoc::viz::viz_file_name;
use lazy_static::lazy_static;
use nannou::color::{Alpha, Hue};
use nannou::glam::f32;
use nannou::lyon::lyon_tessellation::LineCap;
use nannou::prelude::*;

// Sizing
const MARGIN: usize = 20;
const HEIGHT: usize = 1020;
const AVAILABLE: usize = HEIGHT - MARGIN * 2;

// Colors
const C_FIELD: Srgb<u8> = BLACK;
const C_FENCE: Srgb<u8> = SADDLEBROWN;
const C_STUBBLE: Srgb<u8> = PERU;

fn main() {
    // aoc::with_input(2024, 12, do_solve)
    nannou::app(model).update(update).run();
}

struct Growth {
    stage_count: u8,
    lo: f32,
    hi: f32,
    color: Alpha<Srgb<u8>, f32>,
}

impl Growth {
    pub(crate) fn jitter(&self) -> Point2 {
        pt2(
            random_range(self.lo, self.hi),
            random_range(self.lo, self.hi),
        )
    }

    pub(crate) fn jitters(&self) -> Vec<Point2> {
        (0..self.stage_count).map(|_| self.jitter()).collect()
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

lazy_static! {
    static ref PLANTS: Vec<Plant> = ('A'..='Z')
        .map(|c| Plant {
            label: format!("{c}"),
            sprout: Growth::new(GREEN, 30.0, random_range(4, 8), 0.2),
            flower: Growth::new(RED, 70.0, random_range(6, 12), 0.5),
        })
        .collect();
}

struct Plot {
    x: usize,
    y: usize,
    c: usize,
    plant: &'static Plant,
    sprout_jitters: Vec<Point2>,
    flower_jitters: Vec<Point2>,
}

impl Plot {
    fn new(x: usize, y: usize, c: usize) -> Plot {
        let plant = &PLANTS[c];
        Plot {
            x,
            y,
            c,
            plant,
            sprout_jitters: plant.sprout.jitters(),
            flower_jitters: plant.flower.jitters(),
        }
    }
}

#[derive(Default)]
struct Model {
    // static
    full_grid: Vec<Vec<usize>>,
    offset: f32,
    // for update
    paused: bool,
    fully_updated: bool,
    complete: bool,
    last_tick: u128,
    frame: usize,
    final_scale: f32,
    // for view
    clear: bool,
    scale: f32,
    font_size: u32,
    fence_weight: f32,
    to_draw: Vec<Plot>,
    grid: Vec<Vec<usize>>,
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
        .rev()
        .map(|l| l.chars().map(|c| c as usize - 'A' as usize).collect())
        .collect();
    Model {
        offset: -1.0 * (HEIGHT / 2 - MARGIN) as f32,
        final_scale: (AVAILABLE / grid.len()) as f32,
        full_grid: grid,
        ..Model::default()
    }
}

fn key_pressed(app: &App, model: &mut Model, key: Key) {
    match key {
        Key::P => model.paused = !model.paused,
        Key::Q => app.quit(),
        Key::S => {
            app.main_window().capture_frame(viz_file_name("png"));
        }
        _ => {}
    }
}

fn update(_: &App, model: &mut Model, update: Update) {
    const FIRST_ZOOM_FRAME: usize = 10;
    const LAST_ZOOM_FRAME: usize = 50;
    if model.complete {
        return;
    }
    let tick = update.since_start.as_millis();
    if model.paused || (model.frame < FIRST_ZOOM_FRAME && tick - model.last_tick < 60) {
        return;
    }
    if model.fully_updated {
        model.complete = true;
    }
    // println!("{:3}: {:3} ms", model.frame, tick - model.last_tick);
    model.frame += 1;
    model.last_tick = tick;
    let frame = model.frame;
    model.clear = frame <= LAST_ZOOM_FRAME;
    if frame > LAST_ZOOM_FRAME {
        model.to_draw.clear()
    }
    while model.grid.len() < frame && model.grid.len() < model.full_grid.len() {
        model.grid.push(Vec::new());
    }
    for y in 0..model.grid.len() {
        while model.grid[y].len() < frame - y && model.grid[y].len() < model.full_grid[y].len() {
            let x = model.grid[y].len();
            let c = model.full_grid[y][x];
            model.grid[y].push(c);
            model.to_draw.push(Plot::new(x, y, c));
        }
    }
    if let Some(row) = model.grid.last() {
        if row.len() == model.full_grid.last().unwrap().len() {
            model.fully_updated = true;
        }
    }
    if frame > LAST_ZOOM_FRAME {
        // no need to do the scaling dance anymore
        return;
    }
    const F_FIRST_ZOOM_FRAME: f32 = FIRST_ZOOM_FRAME as f32;
    const F_LAST_ZOOM_FRAME: f32 = LAST_ZOOM_FRAME as f32;
    const F_INITIAL_SCALE: f32 = AVAILABLE as f32 / F_FIRST_ZOOM_FRAME;
    let scale = if frame < FIRST_ZOOM_FRAME {
        F_INITIAL_SCALE
    } else if frame < LAST_ZOOM_FRAME {
        // t: time     : currentIteration
        // b: basis    : startValue
        // c: change   : changeInValue
        // d: duration : totalIterations
        nannou::ease::quad::ease_in_out(
            frame as f32 - F_FIRST_ZOOM_FRAME,
            F_INITIAL_SCALE,
            model.final_scale - F_INITIAL_SCALE,
            F_LAST_ZOOM_FRAME - F_FIRST_ZOOM_FRAME,
        )
    } else {
        model.final_scale
    };
    model.scale = scale;
    model.font_size = (scale as u32).min(500);
    model.fence_weight = 1.0.max(scale / 10.0);
    model.frame = frame;
}

fn view(app: &App, model: &Model, frame: Frame) {
    if model.paused || model.complete || model.to_draw.len() == 0 {
        return;
    }

    // Initialize
    let draw = app.draw().x_y(model.offset, model.offset);
    if model.clear {
        draw.background().color(C_FIELD);
    }
    let draw_fence = |a, b| {
        draw.line()
            .start(a)
            .end(b)
            .weight(model.fence_weight)
            .start_cap(LineCap::Round)
            .end_cap(LineCap::Round)
            .color(C_FENCE)
    };

    // Aliases, for shorter expressions
    let s = model.scale;

    // Fences first
    let bound = pt2(
        model.full_grid[0].len() as f32 * s,
        model.full_grid.len() as f32 * s,
    );
    // todo: don't want the upper bounds until complete...
    // todo: though really they should gather in as the rest of the garden is filled
    for st in [pt2(0.0, 0.0), bound] {
        draw_fence(st, pt2(0.0, bound.y));
        draw_fence(st, pt2(bound.x, 0.0));
    }
    for &Plot { x, y, c, .. } in model.to_draw.iter() {
        let start = pt2(x as f32 * s, y as f32 * s);
        if x > 0 && model.full_grid[y][x - 1] != c {
            draw_fence(start, pt2(start.x, start.y + s));
        }
        if y > 0 && model.full_grid[y - 1][x] != c {
            draw_fence(start, pt2(start.x + s, start.y));
        }
    }

    // Finally, the plants!
    for &Plot { x, y, plant, .. } in model.to_draw.iter() {
        let start = pt2(x as f32 * s + s / 2.0, y as f32 * s + s / 1.5);
        draw.text(&plant.label)
            .xy(start)
            .font_size(model.font_size)
            .color(C_STUBBLE);
    }
    let draw_growth = |get_jitters: fn(&Plot) -> &Vec<Point2>,
                       get_color: fn(&Plant) -> Alpha<Srgb<u8>, f32>| {
        for plot in model.to_draw.iter() {
            let jitters = get_jitters(plot);
            let &Plot { x, y, plant, .. } = plot;
            let color = get_color(plant);
            let start = pt2(x as f32 * s + s / 2.0, y as f32 * s + s / 1.5);
            for &jitter in jitters.iter() {
                draw.text(&plant.label)
                    .xy(start + jitter * s)
                    .font_size(model.font_size)
                    .color(color);
            }
        }
    };
    draw_growth(|plot| &plot.sprout_jitters, |plant| plant.sprout.color);
    draw_growth(|plot| &plot.flower_jitters, |plant| plant.flower.color);

    // Send it!
    draw.to_frame(app, &frame).unwrap();
}
