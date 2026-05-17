// Step 1: Adjacency list
// Step 2: Push source to the queue
// Step 3: Pop from queue and loop thru nodes in the adjacency list
// Step 4: When looping thru the neighbors of a node, if the node is not visited, add to queue and...
// ... mark the node as visited

use std::collections::VecDeque;

pub fn valid_path(n: i32, edges: Vec<Vec<i32>>, source: i32, destination: i32) -> bool {
    // Convert to usize, because this is Rust
    let n = n as usize;
    let source = source as usize;
    let destination = destination as usize;

    // List of visited nodes
    let mut visited = vec![false; n];

    // Adjacency list, then populate
    let mut graph: Vec<Vec<usize>> = vec![Vec::new(); n];

    for edge in edges.iter() {
        let u = edge[0] as usize;
        let v = edge[1] as usize;
        graph[u].push(v);
        graph[v].push(u);
    }

    // Queue, then add source to the queue
    let mut q: VecDeque<usize> = VecDeque::new();

    q.push_back(source);
    visited[source] = true;

    while let Some(node) = q.pop_front() {
        if node == destination {
            return true;
        }

        for &neighbor in graph[node].iter() {
            if !visited[neighbor] {
                visited[neighbor] = true;
                q.push_back(neighbor);
            }
        }
    }

    false
}
