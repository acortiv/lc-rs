pub fn valid_path(n: i32, edges: Vec<Vec<i32>>, source: i32, destination: i32) -> bool {
    if edges.is_empty() {
        return source == destination;
    }

    let n = n as usize;
    let source = source as usize;
    let destination = destination as usize;
    let mut visited = vec![false; n];
    let mut adj_list: Vec<Vec<usize>> = vec![Vec::new(); n];

    for edge in edges.iter() {
        let u = edge[0] as usize;
        let v = edge[1] as usize;
        adj_list[u].push(v);
        adj_list[v].push(u);
    }

    

    dfs(source, &mut visited, &adj_list, destination)
}

fn dfs(vertex: usize, visited: &mut [bool], adj_list: &[Vec<usize>], destination: usize) -> bool {
    visited[vertex] = true;

    // base case, found!:
    if vertex == destination {
        return true;
    }

    for &edge in adj_list[vertex].iter() {
        if !visited[edge]
            && dfs(edge, visited, adj_list, destination) {
                return true;
            }
    }

    false
}
