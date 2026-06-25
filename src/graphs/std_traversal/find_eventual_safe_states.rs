// DFS Coloring Solution
// Color the nodes
// 0: unvisited
// 1: being processed
// 2: determined safe
// the trick is to find the "highest" safe node, and all of its descendents
pub fn eventual_safe_nodes(graph: Vec<Vec<i32>>) -> Vec<i32> {
    let n = graph.len();
    let mut visited = vec![0; n];
    let mut res: Vec<i32> = Vec::new();
    let graph: Vec<Vec<usize>> = graph
        .iter()
        .map(|edges| edges.iter().map(|&edge| edge as usize).collect())
        .collect();

    for i in 0..n {
        dfs(i, &mut visited, &graph);
    }

    res.extend(
        visited
            .iter()
            .enumerate()
            .filter_map(|(idx, &state)| (state == 2).then_some(idx as i32)),
    );
    res
}

fn dfs(node: usize, visited: &mut [u8], graph: &[Vec<usize>]) -> bool {
    match visited[node] {
        1 => return false,
        2 => return true,
        _ => {}
    }

    visited[node] = 1;
    for &neighbor in &graph[node] {
        if !dfs(neighbor, visited, graph) {
            return false;
        }
    }

    visited[node] = 2;
    true
}
