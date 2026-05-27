use std::{cmp::Reverse, collections::BinaryHeap};

pub fn network_delay_time(times: Vec<Vec<i32>>, n: i32, k: i32) -> i32 {
    // Preliminary typecasts
    let n = n as usize;
    let k = k as usize;

    // Adjacency list and visited map
    let mut visited: Vec<bool> = vec![false; n];
    let mut graph = vec![Vec::<(usize, i32)>::new(); n];

    for edge in times.iter() {
        let u = (edge[0] - 1) as usize;
        let v = (edge[1] - 1) as usize;
        let c = edge[2];

        graph[u].push((v, c));
    }

    // Binary heap for traversing the graph
    let mut q: BinaryHeap<(Reverse<i32>, usize)> = BinaryHeap::new();
    q.push((Reverse(0), (k - 1) as usize));

    let mut ans;
    let mut seen = 0;

    // Pop from min-heap to traverse graph
    while let Some((Reverse(cost), node)) = q.pop() {
        if visited[node] {
            continue;
        }

        visited[node] = true;
        ans = cost;
        seen += 1;

        if seen == n {
            return ans;
        }

        for &(next, weight) in graph[node].iter() {
            if !visited[next] {
                q.push((Reverse(cost + weight), next));
            }
        }
    }

    -1
}
