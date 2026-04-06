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

struct UnionFind {
    parents: Vec<usize>,
    ranks: Vec<usize>,
    size: usize,
}

impl UnionFind {
    pub fn new(size: usize) -> Self {
        Self {
            parents: (0..size).collect(),
            ranks: vec![0; size],
            size,
        }
    }

    pub fn find(&mut self, x: usize) -> usize {
        if self.parents[x] != x {
            let root = self.find(self.parents[x]);
            self.parents[x] = root;
        }
        self.parents[x]
    }

    pub fn union_set(&mut self, x: usize, y: usize) {
        let (xset, yset) = (self.find(x), self.find(y));
        if self.ranks[xset] < self.ranks[yset] {
            self.parents[xset] = yset;
        } else if self.ranks[xset] > self.ranks[yset] {
            self.parents[yset] = xset;
        } else {
            self.parents[yset] = xset;
            self.ranks[xset] += 1;
        }
    }
}

fn find_circle_num_uf(is_connected: Vec<Vec<i32>>) -> i32 {
    let n = is_connected.len();
    let mut uf = UnionFind::new(n);
    let mut number_of_components = n;

    for i in 0..n {
        for j in i + 1..n {
            if is_connected[i][j] == 1 && uf.find(i) != uf.find(j) {
                number_of_components -= 1;
                uf.union_set(i, j);
            }
        }
    }

    number_of_components as i32
}

fn find_circle_num_uf_iters(is_connected: Vec<Vec<i32>>) -> i32 {
    let n = is_connected.len();
    let mut uf = UnionFind::new(n);
    let mut number_of_components = n;

    for (i, rows) in is_connected.iter().enumerate() {
        for (j, &connected) in rows.iter().enumerate().skip(i + 1) {
            if connected == 1 && uf.find(i) != uf.find(j) {
                number_of_components -= 1;
                uf.union_set(i, j);
            }
        }
    }

    number_of_components as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dfs_provinces() {
        let graph = vec![vec![1, 1, 0], vec![1, 1, 0], vec![0, 0, 1]];
        let g_dfs = graph.clone();
        let g_uf = graph.clone();
        let res_dfs = find_circle_num_dfs(g_dfs);
        assert_eq!(res_dfs, 2);
        let res_uf = find_circle_num_uf_iters(g_uf);
        assert_eq!(res_uf, 2);
    }
}
