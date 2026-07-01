// Thoughts: DFS to get to every leaf
// Pretend the graph is undirected
// Cross reference the actual directed edges and if it isn't going the way i want, flip and add
// I'm not sure this guarantees we flipped the minimum number of edges though

// More thoughts:
// 0: is the legit edge
// 1: is the fake edge

pub fn min_reorder(n: i32, connections: Vec<Vec<i32>>) -> i32 {
    let n = n as usize;
    let mut graph: Vec<Vec<(usize, u8)>> = vec![Vec::new(); n];
    let mut ans = 0;
    let mut visited: Vec<bool> = vec![false; n];

    for edge in &connections {
        let u = edge[0] as usize;
        let v = edge[1] as usize;

        graph[u].push((v, 0));
        graph[v].push((u, 1));
    }

    visited[0] = true;
    dfs(0, &graph, &mut visited, &mut ans);
    ans
}

fn dfs(node: usize, graph: &[Vec<(usize, u8)>], visited: &mut [bool], ans: &mut i32) {
    for &(neighbor, edge_type) in &graph[node] {
        if !visited[neighbor] {
            if edge_type == 0 {
                *ans += 1;
            }
            visited[neighbor] = true;
            dfs(neighbor, graph, visited, ans);
        }
    }
}
