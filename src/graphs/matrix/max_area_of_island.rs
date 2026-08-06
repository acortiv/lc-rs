// Steps:
// 1) Iterate the graph
// 2) When encountering a 1, dfs using the coordinates as the entry point
// 3) after exiting the dfs, record ans = ans.max(res)
// 4) return res

fn max_area_of_island(grid: Vec<Vec<i32>>) -> i32 {
    const DIRS: [(isize, isize); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];
    let (m, n) = (grid.len(), grid[0].len());

    let mut visited = vec![vec![false; n]; m];
    let mut ans = 0;

    for i in 0..m {
        for j in 0..n {
            if grid[i][j] == 1 && !visited[i][j] {
                visited[i][j] = true;
                let area = dfs((i, j), &DIRS, &grid, &mut visited);
                ans = ans.max(area);
            }
        }
    }

    ans
}

fn dfs(
    coordinates: (usize, usize),
    dirs: &[(isize, isize); 4],
    grid: &[Vec<i32>],
    visited: &mut [Vec<bool>],
) -> i32 {
    let (r, c) = coordinates;
    let (m, n) = (grid.len(), grid[0].len());
    let mut area = 1;

    for &(dr, dc) in dirs {
        let Some(nr) = r.checked_add_signed(dr) else {
            continue;
        };

        let Some(nc) = c.checked_add_signed(dc) else {
            continue;
        };

        if nr >= m || nc >= n {
            continue;
        }

        if grid[nr][nc] == 0 {
            continue;
        }

        if visited[nr][nc] {
            continue;
        }

        visited[nr][nc] = true;
        area += dfs((nr, nc), dirs, grid, visited);
    }

    area
}
