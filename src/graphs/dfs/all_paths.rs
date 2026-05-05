pub fn all_paths_source_target(graph: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut all_paths: Vec<Vec<i32>> = Vec::new();
    let mut curr_path: Vec<i32> = Vec::new();

    dfs(0, &mut curr_path, &mut all_paths, &graph);

    all_paths
}

fn dfs(v: usize, curr_path: &mut Vec<i32>, all_paths: &mut Vec<Vec<i32>>, edges: &[Vec<i32>]) {
    curr_path.push(v as i32);

    if v == edges.len() - 1 {
        all_paths.push(curr_path.clone());
        curr_path.pop();
        return;
    }

    for &edge in edges[v].iter() {
        let edge = edge as usize;
        dfs(edge, curr_path, all_paths, edges);
    }

    curr_path.pop();
}
