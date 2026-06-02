use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub fn minimum_effort_path(heights: Vec<Vec<i32>>) -> i32 {
    // left, right, down, up
    const DIRS: [(isize, isize); 4] = [(0, -1), (0, 1), (-1, 0), (1, 0)];

    let rows = heights.len() as isize;
    let cols = heights[0].len() as isize;
    let mut visited: Vec<Vec<bool>> = vec![vec![false; cols as usize]; rows as usize];
    let mut heap: BinaryHeap<(Reverse<i32>, (isize, isize))> = BinaryHeap::new();

    heap.push((Reverse(0), (0, 0)));

    while let Some((cost, (r, c))) = heap.pop() {
        if r == rows - 1 && c == cols - 1 {
            return cost.0;
        }

        let next_steps: Vec<_> = DIRS
            .iter()
            .map(|(dr, dc)| (r + dr, c + dc))
            .filter(|&(nr, nc)| {
                (nr >= 0 && nc >= 0)
                    && (nr <= rows - 1 && nc <= cols - 1)
                    && (!visited[nr as usize][nc as usize])
            })
            .collect();

        let (ur, uc) = (r as usize, c as usize);

        if visited[ur][uc] {
            continue;
        }

        visited[ur][uc] = true;

        if ur == (rows - 1) as usize && uc == (cols - 1) as usize {
            return cost.0;
        }

        let cost = cost.0;
        for (nr, nc) in next_steps {
            let (nr, nc) = (nr as usize, nc as usize);

            let local_cost = (heights[nr][nc] - heights[ur][uc]).abs();
            heap.push((Reverse(cost.max(local_cost)), (nr as isize, nc as isize)));
        }
    }

    -1
}
