#![allow(dead_code)]
use std::cell::Cell;
use std::time::{Duration, Instant};

#[derive(Debug, Default)]
pub(crate) struct Timing {
    count: Cell<usize>,
    time: Cell<Duration>,
    start: Cell<Option<Instant>>,
}

impl Timing {
    pub(crate) fn enter(&self) {
        if let Some(_) = self.start.replace(Some(Instant::now())) {
            panic!("A timed block is already open for this timing?")
        }
    }

    pub(crate) fn exit(&self) {
        if let Some(i) = self.start.replace(None) {
            self.time.set(self.time.get() + i.elapsed());
            self.count.set(self.count.get() + 1);
        } else {
            panic!("A timed block is not open for this timing?")
        }
    }

    pub(crate) fn run<W>(&self, work: W)
    where
        W: FnOnce() -> (),
    {
        self.enter();
        work();
        self.exit();
    }

    pub(crate) fn apply<T, W>(&self, work: W) -> T
    where
        W: FnOnce() -> T,
    {
        self.enter();
        let r = work();
        self.exit();
        r
    }

    pub(crate) fn invoked(&self) -> bool {
        self.count.get() > 0
    }

    pub(crate) fn total_time(&self) -> Duration {
        self.time.get()
    }
}
