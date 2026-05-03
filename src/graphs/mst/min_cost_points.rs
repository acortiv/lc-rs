use ::std::{cmp::Reverse, collections::BinaryHeap};

// Kruskal's Algorithm
// 1) Calculate the manhattan distance for each node to every other node
// this becomes the edges list. use index of points to represent the node for convenience.
// 2) Sort the list of edges
// 3) total costs

pub fn min_cost_connect_points_kruskal(points: Vec<Vec<i32>>) -> i32 {
    let mut edges: Vec<(usize, usize, i32)> = Vec::new();
    let mut uf = UnionFind::new(points.len());
    for (i, u) in points.iter().enumerate() {
        for (j, v) in points.iter().enumerate().skip(i + 1) {
            let md = (u[0] - v[0]).abs() + (u[1] - v[1]).abs();
            edges.push((i, j, md));
        }
    }

    edges.sort_by_key(|e| e.2);
    let mut min_cost: i32 = 0;
    for &(u, v, c) in edges.iter() {
        if uf.find(u) != uf.find(v) {
            uf.union(u, v);
            min_cost += c;
        }
    }

    min_cost
}

struct UnionFind {
    parents: Vec<usize>,
    ranks: Vec<usize>,
}

impl UnionFind {
    pub fn new(size: usize) -> Self {
        Self {
            parents: (0..size).collect(),
            ranks: vec![0; size],
        }
    }

    pub fn find(&mut self, x: usize) -> usize {
        if x != self.parents[x] {
            let root = self.find(self.parents[x]);
            self.parents[x] = root;
        }
        self.parents[x]
    }

    pub fn union(&mut self, x: usize, y: usize) {
        let (rx, ry) = (self.find(x), self.find(y));
        if rx != ry {
            if self.ranks[rx] < self.ranks[ry] {
                self.parents[rx] = ry;
            } else if self.ranks[rx] > self.ranks[ry] {
                self.parents[ry] = rx;
            } else {
                self.parents[ry] = rx;
                self.ranks[rx] += 1;
            }
        }
    }
}

// Prim's algorithm:
// 1) pick an arbitrary node and find the cheapest edge to join to another node
// 2) consider those nodes visited and a set, and the nodes not visited another disconnected set
// 3) find the next cheapest edge to join the visited edge

fn m_value(xi: i32, xj: i32, yi: i32, yj: i32) -> i32 {
    (xi - xj).abs() + (yi - yj).abs()
}

pub fn min_cost_connect_points_prim_not_optimized(points: Vec<Vec<i32>>) -> i32 {
    let n = points.len();

    if n <= 1 {
        return 0;
    }

    let mut p_queue: BinaryHeap<(Reverse<i32>, usize)> = BinaryHeap::new();
    let mut visited = vec![false; n];
    let mut count = n - 1;
    for j in 1..n {
        let cost = m_value(points[0][0], points[j][0], points[0][1], points[j][1]);
        p_queue.push((Reverse(cost), j));
    }

    visited[0] = true;
    let mut result = 0;

    while let Some((c, v)) = p_queue.pop() {
        if count == 0 {
            break;
        }

        if !visited[v] {
            result += c.0;
            visited[v] = true;
            for j in 0..n {
                if !visited[j] {
                    let cost = m_value(points[v][0], points[j][0], points[v][1], points[j][1]);
                    p_queue.push((Reverse(cost), j));
                }
            }
            count -= 1;
        }
    }
    result
}

// Prim's optimized
pub fn min_cost_connect_points(points: Vec<Vec<i32>>) -> i32 {
    let n = points.len();
    let mut visited = vec![false; n];
    let mut min_cost = vec![i32::MAX; n];

    let mut result = 0;
    min_cost[0] = 0;

    for _ in 0..n {
        let mut u = usize::MAX;

        for i in 0..n {
            if !visited[i] && (u == usize::MAX || min_cost[i] < min_cost[u]) {
                u = i;
            }
        }

        visited[u] = true;
        result += min_cost[u];
        for v in 0..n {
            if !visited[v] {
                let cost = m_value(points[u][0], points[v][0], points[u][1], points[v][1]);
                min_cost[v] = min_cost[v].min(cost);
            }
        }
    }

    result
}
