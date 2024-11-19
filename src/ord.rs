/// Auto-implements `PartialOrd`, `PartialEq`, and `Eq` consistently with the
/// `cmp` method the `Ord` trait specifies. At its simplest:
///
/// ```
/// # use std::cmp::Ordering;
/// # use aoc::impl_ord;
/// struct EvenFirst(i32);
///
/// impl_ord!(
///     EvenFirst,
///     // the single item Ord requires...
///     fn cmp(&self, other: &Self) -> Ordering {
///         (self.0 % 2).cmp(&(other.0 % 2)).then(self.0.cmp(&other.0))
///     }
/// );
/// ```
#[macro_export]
macro_rules! impl_ord {
    ($type:ty, $cmp_fn:item) => {
        impl Eq for $type {}

        impl PartialEq for $type {
            fn eq(&self, other: &Self) -> bool {
                self.cmp(other) == Ordering::Equal
            }
        }

        impl PartialOrd for $type {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                Some(self.cmp(other))
            }
        }

        impl Ord for $type {
            $cmp_fn
        }
    };
}

pub use impl_ord;

#[cfg(test)]
mod test {
    use std::cmp::Ordering;

    /// This is bad - the derived traits are inconsistent with the `Ord` impl.
    #[derive(PartialOrd, PartialEq, Eq)]
    struct Even1(i32);

    impl Ord for Even1 {
        fn cmp(&self, other: &Self) -> Ordering {
            (self.0 % 2).cmp(&(other.0 % 2)).then(self.0.cmp(&other.0))
        }
    }

    /// This is good - the macro will impl traits consistent with the `Ord`.
    struct Even2(i32);

    impl_ord!(
        Even2,
        fn cmp(&self, other: &Self) -> Ordering {
            (self.0 % 2).cmp(&(other.0 % 2)).then(self.0.cmp(&other.0))
        }
    );

    /// Because [`Even1`] doesn't implement `PartialOrd` (what `sort` uses), the
    /// fact that it implements `Ord` is irrelevant.
    #[test]
    fn even1() {
        let mut twos = vec![Even1(1), Even1(2), Even1(3), Even1(4), Even1(5)];
        twos.sort_unstable();
        assert_eq!(
            vec![1, 2, 3, 4, 5],
            twos.iter().map(|v| v.0).collect::<Vec<_>>()
        )
    }

    /// Because [`Even2`] auto-implements `PartialOrd` (along with `PartialEq`
    /// and `Eq`), it's `Ord` implementation will propagate to those traits'
    /// uses, including by `sort`.
    #[test]
    fn even2() {
        let mut twos = vec![Even2(1), Even2(2), Even2(3), Even2(4), Even2(5)];
        twos.sort_unstable();
        assert_eq!(
            vec![2, 4, 1, 3, 5],
            twos.iter().map(|v| v.0).collect::<Vec<_>>()
        )
    }
}
