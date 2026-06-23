use std::collections::VecDeque;

pub fn find_min_height_trees(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
    let n = n as usize;
    let mut graph = vec![Vec::new(); n];
    let mut in_degree: Vec<i32> = vec![0; n];

    for edge in &edges {
        let u = edge[0] as usize;
        let v = edge[0] as usize;

        graph[u].push(v);
        in_degree[u] += 1;

        graph[v].push(u);
        in_degree[v] += 1;
    }

    let mut q = VecDeque::new();
    let mut min_height = 0;
    let mut ans = Vec::new();

    for i in 0..n {
        let mut curr_in_degree = in_degree.clone();
        q.push_back(i);

        while let Some(node) = q.pop_front() {
            for &neighbor in &graph[node] {
                curr_in_degree[neighbor] -= 1;
                if curr_in_degree[neighbor] == 0 {
                    q.push_back(neighbor)
                }
            }
        }
    }

    ans
}
