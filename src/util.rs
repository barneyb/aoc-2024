/// Return an [Option] containing the index of the first element in the [Vec]
/// which the predicate accepts, if one exists.
///
/// ```
/// # use aoc::util::index_of_first;
/// let items = vec![1, 2, 2, 3, 3, 3];
///
/// assert_eq!(Some(3), index_of_first(&items, |&it| it > 2));
/// assert_eq!(None, index_of_first(&items, |&it| it < 0));
/// ```
pub fn index_of_first<T, P>(arr: &Vec<T>, predicate: P) -> Option<usize>
where
    P: Fn(&T) -> bool,
{
    // this gets about 10x faster in release builds
    arr.iter()
        .enumerate()
        .filter(|(_, it)| predicate(it))
        .map(|(i, _)| i)
        .next()
}
