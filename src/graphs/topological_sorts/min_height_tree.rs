// We need to track the height of the tree in some capacity...
// In-degrees - Wondering how we should handle these

use std::collections::VecDeque;

pub fn find_min_height_trees(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
    let n = n as usize;
    if n == 1 {
        return vec![0];
    }

    let mut graph = vec![Vec::new(); n];
    let mut degree: Vec<i32> = vec![0; n];
    let mut queue: VecDeque<usize> = VecDeque::new();

    // build the graph and count degrees of each node
    for edge in &edges {
        let u = edge[0] as usize;
        let v = edge[1] as usize;

        graph[u].push(v);
        degree[u] += 1;

        graph[v].push(u);
        degree[v] += 1;
    }

    // Add the leaf nodes (only 1 edge) to the queue to be process and 'peeled'
    queue.extend(
        degree
            .iter()
            .enumerate()
            .filter_map(|(idx, &d)| (d == 1).then_some(idx)),
    );

    let mut remaining = n;

    while remaining > 2 {
        // The height of a graph/tree can be derived by getting the current length of the queue
        let layer_size = queue.len();

        // Subtract the current size of the layer to get the number of remaining nodes
        remaining -= layer_size;

        for _ in 0..layer_size {
            let leaf = queue.pop_front().unwrap();

            for &neighbor in &graph[leaf] {
                degree[neighbor] -= 1;

                if degree[neighbor] == 1 {
                    queue.push_back(neighbor);
                }
            }
        }
    }

    queue.into_iter().map(|n| n as i32).collect()
}
