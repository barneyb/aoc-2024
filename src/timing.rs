//! A simple module for tracking counts and timings for pieces of code. At its
//! simplest:
//!
//! ```
//! # use aoc::timing::Timing;
//! # fn calc_of_interest() {}
//! let t = Timing::default();
//!
//! t.enter();
//! calc_of_interest();
//! t.exit();
//!
//! assert_eq!(1, t.exit_count());
//! ```
//!
//! There are also [run](Timing::run) and [apply](Timing::apply) helpers which
//! accept a procedure and a function respectively, and wrap them with
//! [enter](Timing::enter) and [exit](Timing::exit) calls.
//!
//! Finally, there is [ad_hoc](Timing::ad_hoc) for, shockingly, ad hoc use. Pass
//! it a label and procedure, it'll print timing to STDOUT.
#![allow(dead_code)]
use std::cell::Cell;
use std::time::{Duration, Instant};

#[derive(Debug, Default)]
pub struct Timing {
    count: Cell<usize>,
    time: Cell<Duration>,
    start: Cell<Option<Instant>>,
}

impl Timing {
    /// Run the passed procedure inside an ephemeral `Timing`, and print total
    /// time to STDOUT. The example in the [module-level documentation](self)
    /// can be rewritten:
    ///
    /// ```
    /// # use aoc::timing::Timing;
    /// # fn calc_of_interest() {}
    /// Timing::ad_hoc("calc", calc_of_interest);
    /// // prints "calc: 585.666µs"
    /// ```
    pub fn ad_hoc<W>(label: &str, procedure: W)
    where
        W: FnOnce() -> (),
    {
        let t = Timing::default();
        t.run(procedure);
        println!("{label}: {:?}", t.total_time());
    }
}

impl Timing {
    /// Enter this `Timing`. Panic if already inside.
    pub fn enter(&self) {
        if let Some(_) = self.start.replace(Some(Instant::now())) {
            panic!("A timed block is already open for this timing?")
        }
    }

    /// Exit this `Timing`. Panic if not currently inside.
    pub fn exit(&self) {
        if let Some(i) = self.start.replace(None) {
            self.time.set(self.time.get() + i.elapsed());
            self.count.set(self.count.get() + 1);
        } else {
            panic!("A timed block is not open for this timing?")
        }
    }

    /// Run the passed procedure inside this `Timing`. The example in the
    /// [module-level documentation](self) can be rewritten:
    ///
    /// ```
    /// # use aoc::timing::Timing;
    /// # fn calc_of_interest() {}
    /// let t = Timing::default();
    ///
    /// t.run(calc_of_interest);
    /// assert_eq!(1, t.exit_count());
    /// ```
    pub fn run<W>(&self, procedure: W)
    where
        W: FnOnce() -> (),
    {
        self.enter();
        procedure();
        self.exit();
    }

    /// Apply the passed function inside this `Timing`.  The example in the
    /// [module-level documentation](self) can be rewritten:
    ///
    /// ```
    /// # use aoc::timing::Timing;
    /// # fn fn_of_interest() -> i32 { 42 }
    /// let t = Timing::default();
    ///
    /// let result = t.apply(fn_of_interest);
    /// assert_eq!(1, t.exit_count());
    /// ```
    pub fn apply<T, W>(&self, function: W) -> T
    where
        W: FnOnce() -> T,
    {
        self.enter();
        let r = function();
        self.exit();
        r
    }

    /// Whether this `Timing` has ever been exited.
    ///
    /// ```
    /// # use aoc::timing::Timing;
    /// let t = Timing::default();
    ///
    /// assert!(t.is_empty());
    ///
    /// t.run(|| {});
    ///
    /// assert!(!t.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.exit_count() == 0
    }

    /// Count of exits from this `Timing`.
    pub fn exit_count(&self) -> usize {
        self.count.get()
    }

    /// The total time spent within this `Timing`.
    pub fn total_time(&self) -> Duration {
        self.time.get()
    }

    /// The average time spent within this Timing. Panics if [`exit_count`](Self::exit_count) is
    /// larger than [`u32::MAX`].
    pub fn average_time(&self) -> Duration {
        self.time.get() / self.count.get() as u32
    }
}

#[cfg(test)]
mod timing_tests {
    use super::*;
    use std::thread;

    #[test]
    #[should_panic]
    fn double_enter() {
        let t = Timing::default();
        t.enter();
        t.enter();
    }

    #[test]
    #[should_panic]
    fn exit_outside() {
        let t = Timing::default();
        t.exit();
    }

    #[test]
    fn counts() {
        let t = Timing::default();
        t.run(|| {});
        t.run(|| {});
        assert_eq!(2, t.exit_count());
    }

    #[test]
    fn total_time() {
        let t = Timing::default();
        t.run(|| thread::sleep(Duration::from_millis(100)));
        t.run(|| thread::sleep(Duration::from_millis(200)));
        println!("total time: {:?}", t.total_time());
        assert!(t.total_time() > Duration::from_millis(300));
        // this _could_ flake, but hopefully not
        assert!(t.total_time() < Duration::from_millis(400));
    }

    #[test]
    fn average_time() {
        let t = Timing::default();
        t.run(|| thread::sleep(Duration::from_millis(100)));
        t.run(|| thread::sleep(Duration::from_millis(200)));
        println!("average time: {:?}", t.average_time());
        assert!(t.average_time() > Duration::from_millis(150));
        // this _could_ flake, but hopefully not
        assert!(t.average_time() < Duration::from_millis(200));
    }
}
