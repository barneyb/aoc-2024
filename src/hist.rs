//! A [Histogram] type, and an [IntoHistogram] trait for constructing one from
//! anything `IntoIterator`. Includes ASCII-art `Debug` formatting.
use std::borrow::Borrow;
use std::collections::hash_map::{IntoIter, Keys, Values};
use std::collections::HashMap;
use std::fmt;
use std::fmt::Debug;
use std::hash::Hash;
use std::ops::Deref;

/// A `usize`-valued histogram, backed by a [HashMap].
#[derive(Clone, Default)]
pub struct Histogram<T>
where
    T: Eq + Hash,
{
    map: HashMap<T, usize>,
}

impl<T> Eq for Histogram<T> where T: Eq + Hash {}

impl<T> PartialEq for Histogram<T>
where
    T: Eq + Hash,
{
    fn eq(&self, other: &Self) -> bool {
        self.map == other.map
    }
}

impl<T> Histogram<T>
where
    T: Eq + Hash,
{
    pub fn new() -> Histogram<T> {
        Histogram {
            map: HashMap::new(),
        }
    }

    /// Get the count for a given element from the histogram. Returns zero for
    /// unknown elements. If you need to test existence, use `Deref`-coercion to
    /// `HashMap` for `get` or `contains_key`.
    pub fn count<Q: ?Sized>(&self, k: &Q) -> usize
    where
        T: Borrow<Q>,
        Q: Hash + Eq,
    {
        self.map.get(k).map(|&c| c).unwrap_or_default()
    }

    /// Returns an `Iterator` over the buckets in the histogram.
    pub fn buckets(&self) -> Keys<'_, T, usize> {
        self.map.keys()
    }

    /// Returns an `Iterator` over each bucket's count in the histogram.
    pub fn counts(&self) -> Values<'_, T, usize> {
        self.map.values()
    }

    pub fn increment(&mut self, t: T) {
        self.add(t, 1)
    }

    pub fn add(&mut self, t: T, n: usize) {
        *self.map.entry(t).or_default() += n
    }
}

impl<T> Deref for Histogram<T>
where
    T: Eq + Hash,
{
    type Target = HashMap<T, usize>;

    fn deref(&self) -> &Self::Target {
        &self.map
    }
}

/// Renders an ASCII-art bar graph of the histogram, ordering buckets largest
/// to smallest. Same-sized buckets will be ordered by their key.
///
/// Note that some dubious rounding choices are made around bar widths, as the
/// idea is to give a quick summary of values, not render a verifiable chart. If
/// you need the latter, do your own drawing.
/// ```
/// # use aoc::hist::IntoHistogram;
/// println!("{:?}", vec![1, 2, 2, 3].into_histogram());
/// // 2 | ########
/// // 1 | ####
/// // 3 | ####
/// //   +----------
/// //   | 0      2
/// ```
impl<T> Debug for Histogram<T>
where
    T: Debug + Eq + Hash + Ord,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.map.is_empty() {
            return self.map.fmt(f);
        }
        let max = *self.map.values().max().unwrap();
        let width: usize = if max >= 70 {
            70
        } else if max < 8 {
            max * 4
        } else if max < 12 {
            max * 3
        } else if max < 20 {
            max * 2
        } else {
            max
        };
        let factor = width as f32 / max as f32;
        let mut buckets: Vec<_> = self.buckets().collect();
        buckets.sort();
        let mut buckets: Vec<_> = buckets
            .iter()
            .rev()
            .map(|b| {
                let val = self.count(b);
                (format!("{b:?}"), (val as f32 * factor).ceil() as usize)
            })
            .collect();
        buckets.sort_by_key(|(_, v)| *v);
        let len = buckets.iter().map(|(lbl, _)| lbl.len()).max().unwrap();
        let bar = format!("{:>len$} +-{}-", "", "-".repeat(width));
        let scale = format!("{:>len$} | 0{max:>1$}", "", width - 1);
        if self.map.len() > 8 {
            writeln!(f, "{scale}")?;
            writeln!(f, "{bar}")?;
        }
        for (lbl, val) in buckets.iter().rev() {
            writeln!(f, "{lbl:>len$} | {}", "#".repeat(*val))?;
        }
        writeln!(f, "{bar}")?;
        writeln!(f, "{scale}")?;
        Ok(())
    }
}

/// Conversion into a [Histogram].
///
/// ```
/// # use aoc::hist::IntoHistogram;
/// let hist = vec![1, 2, 2, 3].into_histogram();
///
/// assert_eq!(2, hist.count(&2));
/// assert_eq!(0, hist.count(&42));
/// assert_eq!(None, hist.get(&42));
/// ```
pub trait IntoHistogram {
    /// The type of the elements to bucket, which must be `Eq` + `Hash`, as they
    /// will be used as map keys.
    type Item: Eq + Hash;

    /// Creates a histogram from the value.
    fn into_histogram(self) -> Histogram<Self::Item>;
}

impl<I> IntoHistogram for I
where
    I: IntoIterator,
    I::Item: Eq + Hash,
{
    type Item = I::Item;

    fn into_histogram(self) -> Histogram<Self::Item> {
        Histogram::from_iter(self)
    }
}

impl<T> FromIterator<T> for Histogram<T>
where
    T: Eq + Hash,
{
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut hist = Histogram::new();
        for t in iter {
            hist.increment(t)
        }
        hist
    }
}

impl<T> IntoIterator for Histogram<T>
where
    T: Eq + Hash,
{
    type Item = (T, usize);
    type IntoIter = IntoIter<T, usize>;

    fn into_iter(self) -> Self::IntoIter {
        self.map.into_iter()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn debug_print_test() {
        let mut nums = vec![1, 3, 3, 4];
        println!("{:?}", (&nums).into_histogram());
        for _ in 0..25 {
            nums.push(2);
            nums.push(2);
            nums.push(2);
            println!("{:?}", (&nums).into_histogram());
        }
    }

    #[test]
    fn test_buckets() {
        let hist = vec![1, 1, 2, 3, 3].into_histogram();
        let buckets = hist.buckets().collect::<HashSet<_>>();

        assert_eq!(3, buckets.len());
        assert!(buckets.contains(&1));
        assert!(buckets.contains(&2));
        assert!(buckets.contains(&3));
    }

    #[test]
    fn test_counts() {
        let hist = vec![1, 1, 2, 3].into_histogram();
        let mut counts = hist.counts().collect::<Vec<_>>();
        counts.sort();

        assert_eq!(3, counts.len());
        assert_eq!(1, *counts[0]);
        assert_eq!(1, *counts[1]);
        assert_eq!(2, *counts[2]);
    }

    #[test]
    fn test_increment() {
        let mut hist = Histogram::new();
        hist.increment("rabbit");

        assert_eq!(0, hist.count("cow"));
        assert_eq!(1, hist.count("rabbit"));
    }

    #[test]
    fn test_add() {
        let mut hist = Histogram::new();
        hist.add("rabbit", 40);
        hist.add("rabbit", 2);

        assert_eq!(42, hist.count("rabbit"));
    }
}
