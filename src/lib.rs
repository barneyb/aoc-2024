use console::{style, Style};
use std::io::Error;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::mpsc::{channel, Receiver, RecvError, Sender};
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};
use std::{io, thread};

pub mod aocd;
pub mod block_print;
#[macro_use]
pub mod ord;
pub mod geom2d;
pub mod timing;
pub mod y2015;
pub mod y2016;
pub mod y2017;
pub mod y2018;
pub mod y2019;
pub mod y2020;
pub mod y2021;
pub mod y2022;
pub mod y2023;
pub mod y2024;

#[derive(Debug)]
pub enum Part {
    A(String),
    B(String),
    Parse(String),
    Other(String),
}

/// Invokes the passed `work`, passing it the given year/day/s input as a
/// `String`, and a `Sender` which accepts [Part]-wrapped answers to be printed
/// and verified.
pub fn with_input<S>(year: u32, day: u8, work: S) -> Result<(), Error>
where
    S: FnOnce(&str, Sender<Part>) -> (),
    S: Send + 'static,
{
    let (print_tx, print_rx) = channel();
    let print_handle = thread::spawn(move || {
        let print = Print::new();
        let mut correct = true;
        loop {
            match print_rx.recv() {
                Ok((p, dur)) => correct &= print.print(year, day, &p, dur),
                Err(RecvError) => break,
            }
        }
        correct
    });

    let input = aocd::get_input(year, day)?;
    let (solve_tx, solve_rx) = channel();
    let time_arc = Arc::new(RwLock::new(Instant::now()));
    let answer_handle = {
        let answer_time = time_arc.clone();
        thread::spawn(move || listen_for_answers(answer_time, solve_rx, print_tx))
    };
    {
        let mut t = time_arc.write().unwrap();
        *t = Instant::now();
    }
    work(input.trim_end_matches('\n'), solve_tx);

    answer_handle
        .join()
        .expect("Answer thread should have exited cleanly");
    if print_handle
        .join()
        .expect("Print thread should have exited cleanly")
    {
        Ok(())
    } else {
        Err(Error::new(
            io::ErrorKind::Other,
            "Incorrect answer(s) provided.",
        ))
    }
}

fn listen_for_answers(
    time: Arc<RwLock<Instant>>,
    solve_rx: Receiver<Part>,
    print_tx: Sender<(Part, Duration)>,
) {
    loop {
        match solve_rx.recv() {
            Ok(p) => {
                let dur = {
                    let mut t = time.write().unwrap();
                    let e = t.elapsed();
                    *t = Instant::now();
                    e
                };
                print_tx.send((p, dur)).unwrap()
            }
            Err(RecvError) => break,
        }
    }
}

fn submit(year: u32, day: u8, part: &Part) -> io::Result<bool> {
    if let Part::A(_) | Part::B(_) = part {
        if aocd::submit_answer(year, day, part)? {
            println!("{}", style(format!("Verified {part:?}")).green(),);
            Ok(true)
        } else {
            println!("{}", style(format!("Failed {part:?}")).red());
            Ok(false)
        }
    } else {
        Ok(true)
    }
}

struct Print {
    correct_style: Style,
    wrong_style: Style,
    ans_style: Style,
    parse_style: Style,
    other_style: Style,
    time_style: Style,
    ans_count: AtomicUsize,
}

impl Print {
    fn new() -> Print {
        Print {
            correct_style: Style::new().on_green(),
            wrong_style: Style::new().on_red(),
            parse_style: Style::new().on_blue(),
            other_style: Style::new().on_yellow(),
            ans_style: Style::new().underlined(),
            time_style: Style::new().dim(),
            ans_count: AtomicUsize::new(0),
        }
    }

    fn print(&self, year: u32, day: u8, part: &Part, duration: Duration) -> bool {
        // This is a bit aggro, but whatever.
        self.ans_count.fetch_add(1, Ordering::SeqCst);
        let count = self.ans_count.load(Ordering::SeqCst);
        let correct =
            submit(year, day, part).expect("Answer should submit without error, valid or not.");
        let pstyle = if correct {
            &self.correct_style
        } else {
            &self.wrong_style
        };
        let (ans, lbl) = match part {
            Part::A(a) => (a, pstyle.apply_to("Part A:".to_string())),
            Part::B(a) => (a, pstyle.apply_to("Part B:".to_string())),
            Part::Parse(a) => (a, self.parse_style.apply_to(format!("Parse:"))),
            Part::Other(a) => (a, self.other_style.apply_to(format!("Answer {count}:"))),
        };
        if ans.contains('\n') {
            let twelve_spaces = format!(" {:>12}", "");
            println!(
                "{:>12}{} {}\n{}{}",
                self.correct_style.apply_to(lbl),
                twelve_spaces,
                self.time_style.apply_to(format!("({:>12?})", duration)),
                twelve_spaces,
                ans.replace('\n', &format!("\n{twelve_spaces}"))
            );
        } else {
            println!(
                "{:>12} {:>12} {}",
                self.correct_style.apply_to(lbl),
                self.ans_style.apply_to(ans),
                self.time_style.apply_to(format!("({:>12?})", duration))
            );
        }
        correct
    }
}
