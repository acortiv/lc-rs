use std::collections::VecDeque;
pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
    // preliminary
    let num_courses = num_courses as usize;
    let mut ans: Vec<i32> = Vec::new();

    // adjacency list
    let mut adj_list: Vec<Vec<usize>> = vec![Vec::new(); num_courses];

    // in-degrees
    let mut in_degree: Vec<i32> = vec![0; num_courses];

    // q for nodes with an in-degree of 0

    for req in prerequisites.iter() {
        let u = req[1] as usize;
        let v = req[0] as usize;
        adj_list[u].push(v);
        in_degree[v] += 1;
    }

    let mut q = in_degree
        .iter()
        .enumerate()
        .filter_map(|(i, &d)| (d == 0).then_some(i))
        .collect::<VecDeque<usize>>();

    while let Some(node) = q.pop_front() {
        ans.push(node as i32);

        for &out in adj_list[node].iter() {
            in_degree[out] -= 1;
            if in_degree[out] == 0 {
                q.push_back(out);
            }
        }
    }

    if ans.len() == num_courses {
        ans
    } else {
        Vec::new()
    }
}
