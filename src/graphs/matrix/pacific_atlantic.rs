pub fn pacific_atlantic(heights: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let m = heights.len();
    let n = heights[0].len();
    let mut pacific = vec![vec![false; n]; m];
    let mut atlantic = vec![vec![false; n]; m];

    // left/right borders
    for r in 0..m {
        dfs(r, 0, &heights, &mut pacific);
        dfs(r, n - 1, &heights, &mut atlantic);
    }

    // top/bottom borders
    for c in 0..n {
        dfs(0, c, &heights, &mut pacific);
        dfs(m - 1, c, &heights, &mut atlantic);
    }

    let mut result = Vec::new();

    for r in 0..m {
        for c in 0..n {
            if pacific[r][c] && atlantic[r][c] {
                result.push(vec![r as i32, c as i32]);
            }
        }
    }

    result
}

// Ocean flag: 0 => Atlantic, 1 => Pacific
fn dfs(r: usize, c: usize, heights: &[Vec<i32>], visited: &mut [Vec<bool>]) {
    if visited[r][c] {
        return;
    }

    visited[r][c] = true;

    const DIRS: [(isize, isize); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

    let m = heights.len();
    let n = heights[0].len();

    for (dr, dc) in DIRS {
        let Some(nr) = r.checked_add_signed(dr) else {
            continue;
        };
        let Some(nc) = c.checked_add_signed(dc) else {
            continue;
        };

        if nr >= m || nc >= n {
            continue;
        }

        if heights[nr][nc] < heights[r][c] {
            continue;
        }

        dfs(nr, nc, heights, visited);
    }
}
