use std::collections::VecDeque;

pub fn minimum_semesters(n: i32, relations: Vec<Vec<i32>>) -> i32 {
    let n = n as usize;

    let mut graph: Vec<Vec<usize>> = vec![Vec::new(); n];
    let mut in_degree: Vec<i32> = vec![0; n];
    let mut queue: VecDeque<usize> = VecDeque::new();

    for edge in &relations {
        let u = (edge[0] - 1) as usize;
        let v = (edge[1] - 1) as usize;
        graph[u].push(v);
        in_degree[v] += 1;
    }

    queue.extend(
        in_degree
            .iter()
            .enumerate()
            .filter_map(|(idx, &d)| (d == 0).then_some(idx)),
    );

    let mut height = 0;

    while !queue.is_empty() {
        let layer_size = queue.len();

        for _ in 0..layer_size {
            let node = queue.pop_front().unwrap();

            for &neighbor in &graph[node] {
                in_degree[neighbor] -= 1;
                if in_degree[neighbor] == 0 {
                    queue.push_back(neighbor);
                }
            }
        }

        height += 1
    }

    if in_degree.iter().filter(|&&n| n == 0).count() < n {
        -1
    } else {
        height
    }
}
