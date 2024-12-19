pub fn fill_grid_with_true(rows: usize, cols: usize, mut num_true: usize) -> Vec<Vec<bool>> {
    let mut grid = vec![vec![false; cols]; rows];

    if num_true > rows * cols {
        println!("Number of true values exceeds grid size.");
        num_true = rows * cols;
    }

    let positions: Vec<(usize, usize)> = (0..rows)
        .flat_map(|r| (0..cols).map(move |c| (r, c)))
        .collect();

    for &(r, c) in positions.iter().take(num_true) {
        grid[r][c] = true;
    }

    grid
}

pub fn truncate(s: String, max_length: usize) -> String {
    if s.len() <= max_length {
        s.to_string()
    } else {
        format!("{}...", &s[..max_length - 3])
    }
}
