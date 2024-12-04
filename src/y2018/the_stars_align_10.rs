use crate::block_print::{parse_block_letters, BLOCK};
use crate::Part;
use std::collections::HashSet;
use std::sync::mpsc::Sender;

pub fn do_solve(input: &str, tx: Sender<Part>) {
    let (word, tick_count) = both_parts(input);
    tx.send(Part::A(word)).unwrap();
    tx.send(Part::B(tick_count.to_string())).unwrap();
}

type XY = (isize, isize);

fn both_parts(input: &str) -> (String, usize) {
    let mut points: Vec<(XY, XY)> = input
        .lines()
        .map(|l| {
            let parts: Vec<_> = l.split(['<', ',', '>']).collect();
            let parts = [parts[1], parts[2], parts[4], parts[5]];
            let parts: Vec<isize> = parts.iter().map(|n| n.trim().parse().unwrap()).collect();
            ((parts[0], parts[1]), (parts[2], parts[3]))
        })
        .collect();
    let mut tick_count = 0;
    let mut bounds = compute_bounds(&points);
    let mut area = compute_area(&bounds);
    loop {
        let next = tick(&points);
        let next_bounds = compute_bounds(&next);
        let next_area = compute_area(&next_bounds);
        if next_area > area {
            let (min, max) = bounds;
            let pos: HashSet<_> = points.into_iter().map(|(p, _)| p).collect();
            let mut buffer = String::new();
            for y in min.1..=max.1 {
                for x in min.0..=max.0 {
                    buffer.push(if pos.contains(&(x, y)) { BLOCK } else { '.' });
                }
                buffer.push('\n');
            }
            println!("{buffer}");
            break (parse_block_letters(&buffer).unwrap(), tick_count);
        }
        tick_count += 1;
        points = next;
        area = next_area;
        bounds = next_bounds;
    }
}

fn tick(points: &Vec<(XY, XY)>) -> Vec<(XY, XY)> {
    points
        .iter()
        .map(|((px, py), (dx, dy))| ((px + dx, py + dy), (*dx, *dy)))
        .collect()
}

fn compute_area(bounds: &(XY, XY)) -> isize {
    let ((x1, y1), (x2, y2)) = bounds;
    (x2 - x1 + 1) * (y2 - y1 + 1)
}

fn compute_bounds(points: &Vec<(XY, XY)>) -> (XY, XY) {
    points.iter().fold(
        ((isize::MAX, isize::MAX), (isize::MIN, isize::MIN)),
        |bs, p| {
            (
                (bs.0 .0.min(p.0 .0), bs.0 .1.min(p.0 .1)),
                (bs.1 .0.max(p.0 .0), bs.1 .1.max(p.0 .1)),
            )
        },
    )
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_1: &str = r#"position=< 9,  1> velocity=< 0,  2>
position=< 7,  0> velocity=<-1,  0>
position=< 3, -2> velocity=<-1,  1>
position=< 6, 10> velocity=<-2, -1>
position=< 2, -4> velocity=< 2,  2>
position=<-6, 10> velocity=< 2, -2>
position=< 1,  8> velocity=< 1, -1>
position=< 1,  7> velocity=< 1,  0>
position=<-3, 11> velocity=< 1, -2>
position=< 7,  6> velocity=<-1, -1>
position=<-2,  3> velocity=< 1,  0>
position=<-4,  3> velocity=< 2,  0>
position=<10, -3> velocity=<-1,  1>
position=< 5, 11> velocity=< 1, -2>
position=< 4,  7> velocity=< 0, -1>
position=< 8, -2> velocity=< 0,  1>
position=<15,  0> velocity=<-2,  0>
position=< 1,  6> velocity=< 1,  0>
position=< 8,  9> velocity=< 0, -1>
position=< 3,  3> velocity=<-1,  1>
position=< 0,  5> velocity=< 0, -1>
position=<-2,  2> velocity=< 2,  0>
position=< 5, -2> velocity=< 1,  2>
position=< 1,  4> velocity=< 2,  1>
position=<-2,  7> velocity=< 2, -2>
position=< 3,  6> velocity=<-1, -1>
position=< 5,  0> velocity=< 1,  0>
position=<-6,  0> velocity=< 2,  0>
position=< 5,  9> velocity=< 1, -2>
position=<14,  7> velocity=<-2,  0>
position=<-3,  6> velocity=< 2, -1>"#;

    #[test]
    fn example_1() {
        assert_eq!(("HI".to_string(), 3), both_parts(EXAMPLE_1));
    }

    #[test]
    fn test_real_input() {
        crate::with_input(2018, 10, do_solve).unwrap();
    }
}
