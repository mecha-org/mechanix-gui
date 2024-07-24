/// Repeats all the items of the iterator forever,
/// but returns the cycle number alongside.
/// Inefficient due to all the vectors, but doesn't have to be fast.
pub fn cycle_count<T, I: Clone + Iterator<Item = T>>(iter: I) -> impl Iterator<Item = (T, usize)> {
    let numbered_copies = vec![iter].into_iter().cycle().enumerate();
    numbered_copies.flat_map(|(idx, cycle)|
        // Pair each element from the cycle with a copy of the index.
        cycle.zip(
            vec![idx].into_iter().cycle() // Repeat the index forever.
        ))
}
