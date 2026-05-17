use std::collections::VecDeque;

pub fn all_paths_source_target(graph: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut paths: Vec<Vec<i32>> = Vec::new();
    let mut q: VecDeque<Vec<i32>> = VecDeque::new();
    q.push_back(vec![0]);

    while let Some(path) = q.pop_front() {
        if let Some(&node) = path.last() {
            let node = node as usize;
            if node == graph.len() - 1 {
                paths.push(path);
                continue;
            }

            // &neighbor/*neighbour is i32 of the next neighbor to our current node,
            // we need to create a vec of the current vec + neighbor
            for &neighbor in graph[node].iter() {
                let mut next_path = path.clone();
                next_path.push(neighbor);
                q.push_back(next_path);
            }
        }
    }

    paths
}
