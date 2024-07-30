use super::float_ord::FloatOrd;

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

pub fn find_max_double<T, I, F>(iterator: I, get: F) -> f64
where
    I: Iterator<Item = T>,
    F: Fn(&T) -> f64,
{
    iterator
        .map(|value| FloatOrd(get(&value)))
        .max()
        .unwrap_or(FloatOrd(0f64))
        .0
}
