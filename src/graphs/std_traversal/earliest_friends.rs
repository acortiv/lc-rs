use std::collections::HashSet;

pub fn earliest_acq(logs: Vec<Vec<i32>>, n: i32) -> i32 {
    let mut size = n as usize;
    let mut uf = UnionFind::new(size);
    let mut logs = logs;
    logs.sort_by_key(|log| log[0]);

    for edge in logs.iter() {
        let u = edge[1] as usize;
        let v = edge[2] as usize;
        if uf.find(u) == uf.find(v) {
            // Noop, they've already been unioned
            continue;
        }
        uf.union(u, v);
        size -= 1;
        if size == 1 {
            return edge[0];
        }
    }

    -1
}

struct UnionFind {
    ranks: Vec<usize>,
    roots: Vec<usize>,
}

impl UnionFind {
    pub fn new(size: usize) -> Self {
        Self {
            ranks: vec![0; size],
            roots: (0..size).collect(),
        }
    }

    pub fn find(&mut self, x: usize) -> usize {
        if x != self.roots[x] {
            let root = self.find(self.roots[x]);
            self.roots[x] = root;
        }

        self.roots[x]
    }

    pub fn union(&mut self, x: usize, y: usize) {
        let xr = self.find(x);
        let yr = self.find(y);
        if xr != yr {
            if self.ranks[xr] > self.ranks[yr] {
                self.roots[yr] = xr;
            } else if self.ranks[xr] < self.ranks[yr] {
                self.roots[xr] = yr;
            } else {
                self.roots[yr] = xr;
                self.ranks[xr] += 1;
            }
        }
    }
}
