/// Find maximum f64 value from an iterator
pub fn find_max_double<T, I, F>(iterator: I, get: F) -> f64
where
    I: Iterator<Item = T>,
    F: Fn(&T) -> f64,
{
    iterator
        .map(|value| get(&value))
        .fold(0.0f64, |acc, x| acc.max(x))
}

// pub fn add_offsets<'a, I: 'a, T, F: 'a>(
//     iterator: I,
//     get_size: F,
// ) -> impl Iterator<Item = (f64, T)> + 'a
// where
//     I: Iterator<Item = T>,
//     F: Fn(&T) -> f64,
// {
//     let mut offset = 0.0;
//     iterator.map(move |item| {
//         let size = get_size(&item);
//         let value = (offset, item);
//         offset += size;
//         value
//     })
// }

// pub fn add_offsets<'a, I, T, FKey, FSize>(
//     iterator: I,
//     get_spacing_key: FKey, // &T -> &str (or String)
//     get_size: FSize,       // &T -> f64
//     gap_cfg: &'a crate::config::GapDirection,
// ) -> impl Iterator<Item = (f64, T)> + 'a
// where
//     I: Iterator<Item = T> + 'a,
//     FKey: Fn(&T) -> &str + 'a,
//     FSize: Fn(&T) -> f64 + 'a,
//     T: Clone,
// {
//     let mut offset = 0.0;
//     iterator.map(move |item| {
//         let spacing_key = get_spacing_key(&item);
//         let gap = gap_cfg
//             .custom
//             .get(spacing_key)
//             .cloned()
//             .unwrap_or(gap_cfg.default);
//         let size = get_size(&item);
//         let value = (offset, item.clone());
//         offset += size + gap;
//         value
//     })
// }

pub fn add_offsets<'a, I, T, FKey, FSize>(
    iterator: I,
    get_spacing_key: FKey,
    get_size: FSize,
    gap_cfg: &'a crate::config::GapDirection,
    start_offset: f64, // NEW PARAM: starting position for this row
) -> impl Iterator<Item = (f64, T)> + 'a
where
    I: Iterator<Item = T> + 'a,
    FKey: Fn(&T) -> &str + 'a,
    FSize: Fn(&T) -> f64 + 'a,
    T: Clone,
{
    let mut offset = start_offset;
    iterator.map(move |item| {
        let spacing_key = get_spacing_key(&item);
        let gap = gap_cfg
            .custom
            .get(spacing_key)
            .copied()
            .unwrap_or(gap_cfg.default);
        let size = get_size(&item);
        let value = (offset, item.clone());
        offset += size + gap;
        value
    })
}
