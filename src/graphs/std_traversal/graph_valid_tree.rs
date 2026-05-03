pub fn valid_tree(n: i32, edges: Vec<Vec<i32>>) -> bool {
    let size = n as usize;
    let mut uf = UnionFind::new(size);

    for edge in edges.iter() {
        let u = edge[0] as usize;
        let v = edge[1] as usize;
        // if the nodes have already been joined via a shared ancestor,
        // then this will for a ciruclar reference
        if uf.find(u) == uf.find(v) {
            return false;
        }
        uf.union(u, v);
    }

    // confirm there are no disjoint sets and everything has been unioned
    edges.len() == (n - 1) as usize
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
