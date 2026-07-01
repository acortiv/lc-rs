// I think this is BFS where you measure the size of each layer (where each layer's edges all have the same weight of inform_time[i])
use std::collections::VecDeque;

pub fn num_of_minutes(n: i32, head_id: i32, manager: Vec<i32>, inform_time: Vec<i32>) -> i32 {
    // Only one employee
    let n = n as usize;
    if n == 1 {
        return 0;
    }

    // Entry point
    let head_id = head_id as usize;
    let mut q = VecDeque::new();
    let mut graph: Vec<Vec<usize>> = vec![Vec::new(); n];

    for (idx, &boss) in manager.iter().enumerate() {
        if boss != -1 {
            let boss = boss as usize;
            graph[boss].push(idx);
        }
    }

    let mut ans = 0;
    q.push_back((head_id, 0));

    while let Some((node, acc_time)) = q.pop_front() {
        ans = ans.max(acc_time);

        for &neighbor in &graph[node] {
            q.push_back((neighbor, acc_time + inform_time[node]))
        }
    }

    ans
}
