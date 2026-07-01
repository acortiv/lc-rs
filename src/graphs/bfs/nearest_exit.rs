// Step 1: Create a list of the exits

use std::collections::VecDeque;

pub fn nearest_exit(maze: Vec<Vec<char>>, entrance: Vec<i32>) -> i32 {
    // Up, Down, Right, Left
    // Where (row, column)
    const DIRS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, 1), (0, -1)];

    // Switch entrance type to be easily consumed
    let entrance: (usize, usize, i32) = (entrance[0] as usize, entrance[1] as usize, 0);

    // Get the first, last row of rows/cols in order to determine perimeter coordinates that could potentially be exits
    let row_count = maze.len();
    let column_count = maze[0].len();
    let mut exits: Vec<(usize, usize)> = Vec::new();

    // Build a list of exits
    for row in 0..row_count {
        for col in 0..column_count {
            match (row, col) {
                (row, column)
                    if ((row == 0 || row == row_count - 1)
                        || (col == 0 || col == column_count - 1)) =>
                {
                    if maze[row][column] == '.' {
                        exits.push((row, column))
                    }
                }
                _ => {}
            }
        }
    }

    // Create a queue to process the potential moves that we can make and bfs
    let mut q = VecDeque::new();
    q.push_back(entrance);

    let mut visited = vec![vec![false; column_count]; row_count];

    while let Some((row, col, traveled)) = q.pop_front() {
        if exits.contains(&&(row, col)) && (row, col) != (entrance.0, entrance.1) {
            return traveled;
        }
        let (row, col) = (row as isize, col as isize);
        for &(dr, dc) in DIRS.iter() {
            let (nr, nc) = (row + dr, col + dc);
            if (nr >= 0 && nr < row_count as isize) && (nc >= 0 && nc < column_count as isize) {
                let (nr, nc) = (nr as usize, nc as usize);
                match maze[nr][nc] {
                    '.' if !visited[nr][nc] => {
                        visited[nr][nc] = true;
                        q.push_back((nr, nc, traveled + 1));
                    }
                    _ => {}
                }
            }
        }
    }

    -1
}
