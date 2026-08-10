use std::collections::VecDeque;

// Standard BFS Impelementation
// Left, Right, Up, Down (can't allocate for constants)
const DIRECTIONS: [(isize, isize); 8] = [
    (-1, 0),
    (1, 0),
    (0, 1),
    (0, -1),
    (-1, 1),
    (1, 1),
    (-1, -1),
    (1, -1),
];

pub fn shortest_path_binary_matrix(grid: Vec<Vec<i32>>) -> i32 {
    if grid[0][0] == 1 {
        return -1;
    }

    // down, up, right, left, down + right, down + left, up + right, up + left
    const DIRS: [(isize, isize); 8] = [
        (1, 0),
        (-1, 0),
        (0, 1),
        (0, -1),
        (1, 1),
        (1, -1),
        (-1, 1),
        (-1, -1),
    ];

    let (m, n) = (grid.len(), grid[0].len());
    let mut visited = vec![vec![false; n]; m];
    let mut q = VecDeque::new();

    q.push_back(((0, 0), 1));
    visited[0][0] = true;

    while let Some(((r, c), dist)) = q.pop_front() {
        if (r == m - 1) && (c == n - 1) {
            let ret = if grid[r][c] == 0 { dist } else { -1 };
            return ret;
        }

        for &(dr, dc) in &DIRS {
            let Some(nr) = r.checked_add_signed(dr) else {
                continue;
            };

            let Some(nc) = c.checked_add_signed(dc) else {
                continue;
            };

            if nr >= m || nc >= n {
                continue;
            }

            if visited[nr][nc] {
                continue;
            }

            if grid[nr][nc] == 1 {
                continue;
            }

            visited[nr][nc] = true;
            q.push_back(((nr, nc), dist + 1));
        }
    }

    -1
}

// TODO: Implement the A* Algorithm
use std::cmp::max;
use std::collections::BinaryHeap;

const DIRS: [(i32, i32); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

fn shortest_path_binary_matrix_a(grid: Vec<Vec<i32>>) -> i32 {
    if grid[0][0] == 1 {
        return -1;
    }
    let R = grid.len() as i32;
    let C = grid[0].len() as i32;
    let cheby = |r: i32, c: i32| max((r - (R - 1)).abs(), (c - (C - 1)).abs());
    let mut steps_from = vec![vec![i32::MAX; C as usize]; R as usize];
    steps_from[0][0] = 1;
    // (g+h, g, r, c)
    // g = steps from start, h = heuristic
    let mut pq = BinaryHeap::new();
    pq.push((-1 - cheby(0, 0), 1, 0, 0));
    while let Some((_, g, r, c)) = pq.pop() {
        if r == R - 1 && c == C - 1 {
            return g;
        }
        if steps_from[r as usize][c as usize] < g {
            continue;
        }
        for (dr, dc) in DIRS {
            let nr = r + dr;
            let nc = c + dc;
            let ng = g + 1;
            if 0 <= nr
                && nr < R
                && 0 <= nc
                && nc < C
                && grid[nr as usize][nc as usize] == 0
                && ng < steps_from[nr as usize][nc as usize]
            {
                steps_from[nr as usize][nc as usize] = ng;
                pq.push((-ng - cheby(nr, nc), ng, nr, nc));
            }
        }
    }

    -1
}
