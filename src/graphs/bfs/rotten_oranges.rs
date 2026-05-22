use std::collections::VecDeque;

pub fn oranges_rotting(grid: Vec<Vec<i32>>) -> i32 {
    const DIRS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, 1), (0, -1)];

    // x, y, time_passeed
    let mut q: VecDeque<(usize, usize, i32)> = VecDeque::new();
    let mut visited: Vec<Vec<u8>> = vec![vec![0; grid[0].len()]; grid.len()];
    let mut ripe_oranges = 0;
    let mut time: i32 = 0;

    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == 0 {
                continue;
            } else if grid[i][j] == 1 {
                ripe_oranges += 1
            } else {
                // col, row, time
                q.push_back((i, j, 0));
                visited[i][j] = 1;
            }
        }
    }

    while let Some(rotten_orange) = q.pop_front() {
        // swap for cartesian coordinates
        let x = rotten_orange.1 as isize;
        let y = rotten_orange.0 as isize;
        for (dx, dy) in DIRS.iter() {
            let (nx, ny) = (x + dx, y + dy);
            // Bounds check (x coordinate is out of bounds here for some reason)
            if (nx >= 0 && ny >= 0)
                && (nx <= ((grid[0].len() - 1) as isize) && ny <= ((grid.len() - 1) as isize))
            {
                // Shadow to begin checking options
                let (nx, ny) = (nx as usize, ny as usize);

                if grid[ny][nx] == 1 && visited[ny][nx] == 0 {
                    ripe_oranges -= 1;
                    visited[ny][nx] = 1;
                    let nm = rotten_orange.2 + 1;
                    time = std::cmp::max(time, nm);
                    q.push_back((ny, nx, nm));
                }
            }
        }
    }

    if ripe_oranges == 0 { time } else { -1 }
}
