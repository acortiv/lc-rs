// Shortest path with state
// Build a graph with possible the shape Vec<Vec<(dest, color, total_traversed)>>
use std::collections::VecDeque;

pub fn shortest_alternating_paths(
    n: i32,
    red_edges: Vec<Vec<i32>>,
    blue_edges: Vec<Vec<i32>>,
) -> Vec<i32> {
    let n = n as usize;
    let mut queue: VecDeque<(usize, u8, i32)> = VecDeque::from([(0, 0, 0), (0, 1, 0)]);
    let mut graph: Vec<Vec<(usize, u8)>> = vec![Vec::new(); n];
    let mut visited = vec![[false; 2]; n];
    visited[0][0] = true;
    visited[0][1] = true;

    for edge in &red_edges {
        let u = edge[0] as usize;
        let v = edge[1] as usize;
        graph[u].push((v, 0));
    }

    for edge in &blue_edges {
        let u = edge[0] as usize;
        let v = edge[1] as usize;
        graph[u].push((v, 1));
    }

    let mut ans = vec![-1; n];
    ans[0] = 0;

    while let Some((node, u_color, path_len)) = queue.pop_front() {
        if ans[node] == -1 {
            ans[node] = path_len;
        }

        // have to allow for cycles, !visited[neighbor] i don't think is the check
        for &(neighbor, v_color) in &graph[node] {
            if u_color != v_color && !visited[neighbor][v_color as usize] {
                queue.push_back((neighbor, v_color, path_len + 1));
            }
        }
    }

    ans
}
