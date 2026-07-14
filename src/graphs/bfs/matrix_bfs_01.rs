use std::collections::VecDeque;

pub fn update_matrix(mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    // down, up, right, left, down + right, down + left, up + right, up + left
    const DIRS: [(isize, isize); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

    let m = mat.len();
    let n = mat[0].len();

    // let this serve additionally as visited and ans (i32::MAX == unvisited)
    let mut q = VecDeque::new();
    let mut ans = vec![vec![i32::MAX; n]; m];

    for r in 0..m {
        for c in 0..n {
            if mat[r][c] == 0 {
                ans[r][c] = 0;
                q.push_back((r, c));
            }
        }
    }

    while let Some((r, c)) = q.pop_front() {
        for &(dr, dc) in &DIRS {
            let Some(nr) = r.checked_add_signed(dr) else {
                continue;
            };

            let Some(nc) = c.checked_add_signed(dc) else {
                continue;
            };

            if nr >= m || nc >= n || ans[nr][nc] != i32::MAX {
                continue;
            }

            ans[nr][nc] = ans[r][c] + 1;
            q.push_back((nr, nc))
        }
    }

    ans
}
