// pub const BEZIER_POINTS: [f64; 4] = [0.0, 0.0, 480.0, 480.0];
// fn main() {
//     let inc = 1.0 / 12.0;
//     let dy = 40.0;
//     // Compute the correct `t` for `dy`
//     let t = find_t_for_y(BEZIER_POINTS, dy).unwrap_or(0.0);
//     // Forward translations
//     let forward_start_time = if (dy - BEZIER_POINTS[3]).abs() < 0.1 {
//         1.0
//     } else {
//         t
//     };
//     let trans = get_translations(BEZIER_POINTS, forward_start_time, inc, false);
//     println!("Forward: {:?}", trans);
//     // Reverse translations
//     let reverse_start_time = if (dy - BEZIER_POINTS[0]).abs() < 0.1 {
//         0.0
//     } else {
//         t
//     };
//     let trans_reverse = get_translations(BEZIER_POINTS, reverse_start_time, inc, true);
//     println!("Reverse: {:?}", trans_reverse);
// }
pub fn cubic_bezier(arr: &[f64; 4], t: f64) -> f64 {
    let ut = 1.0 - t;
    let a1 = arr[1] * ut + arr[2] * t;
    let result = ((arr[0] * ut + arr[1] * t) * ut + a1 * t) * ut
        + (a1 * ut + (arr[2] * ut + arr[3] * t) * t) * t;
    round_to_precision(result, 1) // Round to 1 decimal place
}
/// Find the `t` value for a given `y` on the Bézier curve
pub fn find_t_for_y(bezier_points: [f64; 4], y: f64) -> Option<f64> {
    let mut t = 0.0;
    let step = 0.001; // Small step for precision
    let mut closest_t = None;
    let mut closest_diff = f64::MAX;
    while t <= 1.0 {
        let current_y = cubic_bezier(&bezier_points, t);
        let diff = (current_y - y).abs();
        if diff < closest_diff {
            closest_diff = diff;
            closest_t = Some(t);
        }
        // Stop early if we hit a close enough value
        if diff < 0.1 {
            break;
        }
        t += step;
    }
    closest_t
}
pub fn get_translations(
    bezier_points: [f64; 4],
    start_time: f64,
    step: f64,
    reverse: bool,
) -> Vec<f64> {
    let mut time = start_time;
    let mut translations = Vec::new();
    if reverse {
        while time > 0.0 {
            translations.push(cubic_bezier(&bezier_points, time));
            time -= step;
        }
        // Ensure the last value matches the first Bézier point
        if time + step > 0.0 {
            translations.push(bezier_points[0]);
        }
    } else {
        while time < 1.0 {
            translations.push(cubic_bezier(&bezier_points, time));
            time += step;
        }
        // Ensure the last value matches the last Bézier point
        if time - step < 1.0 {
            translations.push(bezier_points[3]);
        }
    }
    translations
}
/// Round a floating-point number to a specified precision
pub fn round_to_precision(value: f64, precision: usize) -> f64 {
    let factor = 10f64.powi(precision as i32);
    (value * factor).round() / factor
}
