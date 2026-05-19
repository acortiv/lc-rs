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
    // origin is grid[0][0] and end is grid[grid.len() - 1][grid[0].len() - 1]
    if grid[0][0] == 1 || grid[grid.len() - 1][grid[0].len() - 1] == 1 {
        return -1;
    }

    // Queue + Visited Map
    let mut q: VecDeque<(isize, isize, i32)> = VecDeque::new();
    let mut visited = vec![vec![0; grid[0].len()]; grid.len()];

    // Add the origin
    q.push_back((0, 0, 1));
    visited[0][0] = 1;

    while let Some(node) = q.pop_front() {
        if (node.0 == ((grid.len() - 1) as isize)) && (node.1 == ((grid[0].len() - 1) as isize)) {
            return node.2;
        }
        for (dx, dy) in DIRECTIONS.iter() {
            let next_x = node.0 + dx;
            let next_y = node.1 + dy;
            if !(next_x <= ((grid.len() - 1) as isize)
                && next_y <= ((grid[0].len() - 1) as isize)
                && (next_x >= 0 && next_y >= 0))
            {
                continue;
            }
            let next_x = next_x as usize;
            let next_y = next_y as usize;
            if grid[next_x][next_y] == 0 && visited[next_x][next_y] == 0 {
                visited[next_x][next_y] = 1;
                q.push_back((next_x as isize, next_y as isize, node.2 + 1));
            }
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
