use std::collections::HashSet;

pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
    const DIRS: [(isize, isize); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

    let (m, n) = (grid.len(), grid[0].len());

    let mut visited = HashSet::new();
    let mut ans = 0;

    for i in 0..m {
        for j in 0..n {
            if grid[i][j] == '1' && !visited.contains(&(i, j)) {
                ans += 1;
                // dfs the graph to record all the nodes that are apart of the island found
                dfs((i, j), &DIRS, &grid, &mut visited);
            }
        }
    }

    ans
}

fn dfs(
    coordinates: (usize, usize),
    dirs: &[(isize, isize); 4],
    grid: &[Vec<char>],
    visited: &mut HashSet<(usize, usize)>,
) {
    if !visited.insert(coordinates) {
        return;
    }

    let (m, n) = (grid.len(), grid[0].len());
    let (r, c) = coordinates;

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

        if grid[nr][nc] != '1' {
            continue;
        }

        dfs((nr, nc), dirs, grid, visited)
    }
}
