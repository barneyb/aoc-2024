use crate::aocd;
use console::{style, Style, StyledObject};
use std::io::Error;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::mpsc::{channel, Receiver, RecvError, Sender};
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};
use std::{env, io, thread};

#[derive(Debug)]
pub enum Part {
    Parse(),
    Parsed(String),
    A(String),
    B(String),
    Both(String, String),
    Other(String),
    Join(),
}

/// Invokes the passed `work`, passing it the given year/day/s input as a
/// `String`, and a `Sender` which accepts [Part]-wrapped answers to be printed
/// and verified.
pub fn with_input<S>(year: u32, day: u8, work: S) -> Result<(), Error>
where
    S: FnOnce(&str, Sender<Part>) -> (),
    S: Send + 'static,
{
    let solve_nanos = env::var("BEB_SOLVE_NANOS")
        .map(|v| v != "0")
        .unwrap_or(false);
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
        let ptx = print_tx.clone();
        let answer_time = time_arc.clone();
        thread::spawn(move || listen_for_answers(answer_time, solve_rx, ptx))
    };
    {
        let mut t = time_arc.write().unwrap();
        *t = Instant::now();
    }

    let solve_nanos_start = Instant::now();
    work(input.trim_end_matches('\n'), solve_tx);
    answer_handle
        .join()
        .expect("Answer thread should have exited cleanly");
    let solve_elapsed = solve_nanos_start.elapsed();
    if solve_nanos {
        println!("¡¡solve nanos {}!!", solve_elapsed.as_nanos());
    } else {
        print_tx.send((Part::Join(), solve_elapsed)).unwrap();
    }
    drop(print_tx); // since cloned above
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
    let mut seen_a = false;
    loop {
        match solve_rx.recv() {
            Ok(p) => {
                let dur = {
                    let mut t = time.write().unwrap();
                    let e = t.elapsed();
                    *t = Instant::now();
                    e
                };
                match &p {
                    Part::A(_) | Part::Both(_, _) => seen_a = true,
                    Part::B(_) if !seen_a => {
                        panic!("Part B can't be answered before part A. Undo the shenanigans.")
                    }
                    _ => {}
                }
                print_tx.send((p, dur)).unwrap()
            }
            Err(RecvError) => break,
        }
    }
}

fn submit(year: u32, day: u8, part: &str, val: &str) -> bool {
    if aocd::submit_answer(year, day, part, val)
        .expect("Answer should submit without error, valid or not.")
    {
        println!("{}", style(format!("Verified {part:?}")).green());
        true
    } else {
        println!("{}", style(format!("Failed {part:?}")).red());
        false
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
            ans_count: AtomicUsize::new(1),
        }
    }

    fn part_style(&self, correct: bool) -> &Style {
        if correct {
            &self.correct_style
        } else {
            &self.wrong_style
        }
    }

    fn print(&self, year: u32, day: u8, part: &Part, duration: Duration) -> bool {
        let count = self.ans_count.load(Ordering::SeqCst);
        let mut correct = true;
        match part {
            Part::A(a) => {
                correct &= submit(year, day, "a", a);
                self.do_print(
                    self.part_style(correct).apply_to("Part A:".to_string()),
                    Some(a),
                    Some(duration),
                )
            }
            Part::B(b) => {
                correct &= submit(year, day, "b", b);
                self.do_print(
                    self.part_style(correct).apply_to("Part B:".to_string()),
                    Some(b),
                    Some(duration),
                )
            }
            Part::Both(a, b) => {
                let ac = submit(year, day, "a", a);
                let bc = submit(year, day, "b", b);
                correct &= ac & bc;
                self.do_print(
                    self.part_style(ac).apply_to("Part A:".to_string()),
                    Some(a),
                    None,
                );
                self.do_print(
                    self.part_style(bc).apply_to("Part B:".to_string()),
                    Some(b),
                    Some(duration),
                )
            }
            Part::Parse() => self.do_print(
                self.parse_style.apply_to("Parsed ".to_string()),
                None,
                Some(duration),
            ),
            Part::Parsed(a) => self.do_print(
                self.parse_style.apply_to("Parse:".to_string()),
                Some(a),
                Some(duration),
            ),
            Part::Other(a) => self.do_print(
                self.other_style.apply_to(format!("Answer {count}:")),
                Some(a),
                Some(duration),
            ),
            Part::Join() => self.do_print(
                self.time_style.apply_to("Join ".to_string()),
                None,
                Some(duration),
            ),
        }
        correct
    }

    fn do_print(
        &self,
        lbl: StyledObject<String>,
        ans: Option<&String>,
        duration: Option<Duration>,
    ) {
        if let Some(_) = ans {
            // This is a bit aggro, but whatever.
            self.ans_count.fetch_add(1, Ordering::SeqCst);
        }
        let styled_duration = self.time_style.apply_to(if let Some(d) = duration {
            format!("({:>12?})", d)
        } else {
            String::from("|")
        });
        match ans {
            None => {
                println!("{:>12} {:>12} {}", lbl, "", styled_duration);
            }
            Some(ans) if ans.contains('\n') => {
                let twelve_spaces = format!("{:>12}", "");
                println!(
                    "{:>12} {} {}\n {}{}",
                    lbl,
                    twelve_spaces,
                    styled_duration,
                    twelve_spaces,
                    ans.replace('\n', &format!("\n {twelve_spaces}"))
                );
            }
            Some(ans) => {
                println!(
                    "{:>12} {:>12} {}",
                    lbl,
                    self.ans_style.apply_to(ans),
                    styled_duration
                );
            }
        }
    }
}
