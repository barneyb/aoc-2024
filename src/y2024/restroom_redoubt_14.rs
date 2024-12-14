use crate::hist::Histogram;
use crate::Part;
use regex::Regex;
use std::collections::HashSet;
use std::sync::mpsc::Sender;

pub fn do_solve(input: &str, tx: Sender<Part>) {
    let bots = parse(input);
    tx.send(Part::A(part_one(&bots).to_string())).unwrap();
    tx.send(Part::B(part_two(&bots).to_string())).unwrap();
}

type Pt = (i64, i64);
type Bot = (Pt, Pt);

fn parse(input: &str) -> Vec<Bot> {
    let re = Regex::new("p=(-?[0-9]+),(-?[0-9]+) v=(-?[0-9]+),(-?[0-9]+)").unwrap();
    input
        .lines()
        .map(|l| re.captures(l).unwrap())
        .map(|c| {
            let ns: Vec<i64> = c
                .iter()
                .map(|m| m.unwrap().as_str())
                .skip(1)
                .map(|n| n.parse().unwrap())
                .collect();
            ((ns[0], ns[1]), (ns[2], ns[3]))
        })
        .collect()
}

const WIDTH: i64 = 101;
const HEIGHT: i64 = 103;

fn part_one(bots: &Vec<Bot>) -> usize {
    // 101 tiles wide and 103 tiles tall
    part_one_parameterized(bots, WIDTH, HEIGHT, 100)
}

fn pass_time(bots: &Vec<Bot>, width: i64, height: i64, ticks: usize) -> Vec<Pt> {
    let ticks = ticks as i64;
    bots.iter()
        .map(|((x, y), (dx, dy))| {
            let mut x = (x + dx * ticks) % width;
            let mut y = (y + dy * ticks) % height;
            if x < 0 {
                x += width
            }
            if y < 0 {
                y += height
            }
            (x, y)
        })
        .collect()
}

fn part_one_parameterized(bots: &Vec<Bot>, width: i64, height: i64, ticks: usize) -> usize {
    let mid_x = width / 2;
    let mid_y = height / 2;
    let mut quads = [0; 4];
    for (x, y) in pass_time(bots, width, height, ticks) {
        if x < mid_x {
            if y < mid_y {
                quads[0] += 1;
            } else if y > mid_y {
                quads[1] += 1;
            }
        } else if x > mid_x {
            if y < mid_y {
                quads[2] += 1;
            } else if y > mid_y {
                quads[3] += 1;
            }
        }
    }
    quads.iter().product()
}

/// I compute the maximum number of occupied tiles in any single column and row
/// at time `t` and return them as a pair.
fn get_most_populated_at(bots: &Vec<Bot>, t: usize) -> (usize, usize) {
    let bots: HashSet<_> = pass_time(bots, WIDTH, HEIGHT, t).into_iter().collect();
    let x = bots.iter().map(|(x, _)| *x).collect::<Histogram<_>>();
    let y = bots.iter().map(|(_, y)| *y).collect::<Histogram<_>>();
    (*x.values().max().unwrap(), *y.values().max().unwrap())
}

#[allow(dead_code)]
fn print_at(bots: &Vec<Bot>, t: usize) {
    let bots: HashSet<_> = pass_time(bots, WIDTH, HEIGHT, t).into_iter().collect();
    let mut buf = String::with_capacity(HEIGHT as usize * (WIDTH as usize + 1));
    for y in 0..HEIGHT {
        if y > 0 {
            buf.push('\n');
        }
        for x in 0..WIDTH {
            buf.push(if bots.contains(&(x, y)) { '#' } else { '.' })
        }
    }
    println!("After {t} seconds:\n{buf}");
}

fn convergence_at(x_offset: usize, y_offset: usize) -> usize {
    // if y's "ahead", it goes around an extra time
    let cycles_behind = if y_offset > x_offset { 2 } else { 1 };
    // how far is y behind?
    let steps_to_catch_up = (WIDTH as usize * cycles_behind + x_offset) - y_offset;
    // two steps per cycle
    let cycles_to_catch_up = steps_to_catch_up / 2;
    cycles_to_catch_up * HEIGHT as usize + y_offset
}

fn part_two(bots: &Vec<Bot>) -> usize {
    // First, find the ticks with the most populated column and row.
    let mut max_x = 0;
    let mut tx = 0;
    let mut max_y = 0;
    let mut ty = 0;
    for t in 0..=(WIDTH.max(HEIGHT) as usize) {
        let (x, y) = get_most_populated_at(bots, t);
        if x > max_x {
            tx = t;
            max_x = x;
        }
        if y > max_y {
            ty = t;
            max_y = y;
        }
    }
    // Then advance time forward until their cycles converge: the tree!
    convergence_at(tx, ty)
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_1: &str = r#"p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3"#;

    #[test]
    fn example_1() {
        // 11 tiles wide and 7 tiles tall
        let bots = parse(EXAMPLE_1);
        assert_eq!(r"12", part_one_parameterized(&bots, 11, 7, 100).to_string());
    }

    #[test]
    fn test_convergence() {
        assert_eq!(7055, convergence_at(86, 51));
        assert_eq!(7584, convergence_at(9, 65));
        assert_eq!(7083, convergence_at(13, 79));
    }

    #[test]
    fn test_real_input() {
        crate::with_input(2024, 14, do_solve).unwrap();
    }
}
