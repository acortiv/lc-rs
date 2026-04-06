fn find_circle_num_dfs(is_connected: Vec<Vec<i32>>) -> i32 {
    let n = is_connected.len();
    let mut visit = vec![false; n];
    let mut number_of_components = 0;

    for i in 0..n {
        if visit[i] == false {
            number_of_components += 1;
            dfs(i, &is_connected, &mut visit);
        }
    }

    number_of_components
}

fn dfs(node: usize, is_connected: &[Vec<i32>], visit: &mut [bool]) {
    visit[node] = true;
    for (neighbor, &connected) in is_connected[node].iter().enumerate() {
        if connected == 1 && visit[neighbor] == false {
            dfs(neighbor, is_connected, visit)
        }
    }
}

fn find_circle_num_bfs(is_connected: Vec<Vec<i32>>) -> i32 {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dfs_provinces() {
        let res = find_circle_num_dfs(vec![vec![1, 1, 0], vec![1, 1, 0], vec![0, 0, 1]]);
        assert_eq!(res, 2);
    }
}
