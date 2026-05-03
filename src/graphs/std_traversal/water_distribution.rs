
pub fn min_cost_to_supply_water(n: i32, wells: Vec<i32>, pipes: Vec<Vec<i32>>) -> i32 {
    let n = n as usize;
    let mut uf = UnionFind::new(n + 1);
    let mut edges: Vec<(usize, usize, i32)> = Vec::new();

    // Well is i32 which implements copy, so we match here
    // for (idx, &well) in wells.iter().enumerate() {
    //     edges.push((0, idx + 1, well))
    // }

    // Create a virtual node for wells being
    // dug with 0 being the vertice.
    for i in 1..=n {
        edges.push((0, i, wells[i - 1]));
    }

    // push pipes which are edges
    for pipe in pipes.iter() {
        edges.push((pipe[0] as usize, pipe[1] as usize, pipe[2]))
    }

    // Sort the list to start with the cheapest edges
    edges.sort_by_key(|e| e.2);
    let mut total_cost: i32 = 0;
    // walk edges
    for &(u, v, c) in edges.iter() {
        // Check to make sure there are no cycles as this
        // is a minimum spanning tree problem
        if uf.find(u) != uf.find(v) {
            uf.union(u, v);
            total_cost += c
        }
    }

    total_cost
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
