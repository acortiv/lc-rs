// Find the perimeters of both islands
// loop thru each perimeter nodes and record the absolute distance abs((x1 - x) + (y1 - y))
// answer is the min of the absolute distances

use std::collections::{HashSet, VecDeque};

pub fn shortest_bridge(grid: Vec<Vec<i32>>) -> i32 {
    // (row, col) - down, up, right, left
    const DIRS: [(isize, isize); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

    // We're a square
    let n = grid.len();

    // find first 1 of first island
    let mut origin = None;

    'outer: for i in 0..n {
        for j in 0..n {
            if grid[i][j] == 1 {
                origin = Some((i, j));
                break 'outer;
            }
        }
    }

    let mut island = HashSet::new();
    dfs(origin.unwrap(), &grid, &mut island, &DIRS);

    let mut q: VecDeque<((usize, usize), i32)> = island.iter().map(|&(r, c)| ((r, c), 0)).collect();

    let mut seen = island.clone();
    while let Some(((r, c), path_len)) = q.pop_front() {
        if grid[r][c] == 1 && !island.contains(&(r, c)) {
            return path_len - 1;
        }

        for &(dr, dc) in &DIRS {
            let (r, c) = (r as isize, c as isize);
            let (nr, nc) = (r + dr, c + dc);
            // Bounds checks
            if nr >= 0 && nr < (n as isize) && nc >= 0 && nc < (n as isize) {
                let (nr, nc) = (nr as usize, nc as usize);
                if !seen.contains(&(nr, nc)) {
                    q.push_back(((nr, nc), path_len + 1));
                    seen.insert((nr, nc));
                }
            }
        }
    }

    -1
}

fn dfs(
    coordinates: (usize, usize),
    grid: &[Vec<i32>],
    visited: &mut HashSet<(usize, usize)>,
    dirs: &[(isize, isize); 4],
) {
    visited.insert(coordinates);
    let n = grid.len();

    let (r, c) = (coordinates.0 as isize, coordinates.1 as isize);
    for &(dr, dc) in dirs {
        let (nr, nc) = (r + dr, c + dc);

        // Bounds checks
        if nr >= 0 && nr < n as isize && nc >= 0 && nc < n as isize {
            let (nr, nc) = (nr as usize, nc as usize);
            if !visited.contains(&(nr, nc)) && grid[nr][nc] == 1 {
                dfs((nr, nc), grid, visited, dirs);
            }
        }
    }
}
