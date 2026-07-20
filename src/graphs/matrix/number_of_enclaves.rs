use std::collections::VecDeque;

pub fn num_enclaves(grid: Vec<Vec<i32>>) -> i32 {
    const DIRS: [(isize, isize); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

    let (m, n) = (grid.len(), grid[0].len());
    let mut q = VecDeque::new();

    let mut visited = vec![vec![false; n]; m];
    let mut ans = 0;

    for i in 0..m {
        for j in 0..n {
            // max or min row
            // max or min column
            if grid[i][j] == 1 {
                ans += 1;
                if (i == 0 || i == (m - 1)) || (j == 0 || j == (n - 1)) {
                    visited[i][j] = true;
                    q.push_back((i, j));
                }
            }
        }
    }

    while let Some((r, c)) = q.pop_front() {
        ans -= 1;

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

            if grid[nr][nc] != 1 {
                continue;
            }

            visited[nr][nc] = true;
            q.push_back((nr, nc));
        }
    }

    ans
}
