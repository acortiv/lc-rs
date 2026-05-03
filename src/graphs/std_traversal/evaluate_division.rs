use std::collections::HashMap;

pub fn calc_equation(
    equations: Vec<Vec<String>>,
    values: Vec<f64>,
    queries: Vec<Vec<String>>,
) -> Vec<f64> {
    let mut ids: HashMap<String, usize> = HashMap::new();
    for op in equations.iter() {
        for var in op.iter() {
            let next_id = ids.len();
            ids.entry(var.clone()).or_insert(next_id);
        }
    }

    let mut uf = UnionFind::new(ids.len());
    for (equation, &value) in equations.iter().zip(values.iter()) {
        let x = ids[&equation[0]];
        let y = ids[&equation[1]];
        uf.union(x, y, value);
    }

    let mut ans = Vec::new();
    for query in queries.iter() {
        let a = ids.get(&query[0]);
        let b = ids.get(&query[1]);

        match (a, b) {
            (Some(&x), Some(&y)) => {
                if uf.find(x) != uf.find(y) {
                    ans.push(-1.0);
                } else {
                    ans.push(uf.weights[x] / uf.weights[y])
                }
            }
            _ => ans.push(-1.0),
        }
    }

    ans
}

struct UnionFind {
    ranks: Vec<usize>,
    roots: Vec<usize>,
    /// Where `weights` is x / parent[x]
    weights: Vec<f64>,
}

impl UnionFind {
    pub fn new(size: usize) -> Self {
        Self {
            ranks: vec![0; size],
            roots: (0..size).collect(),
            weights: vec![1.0; size],
        }
    }

    pub fn find(&mut self, x: usize) -> (usize) {
        if x != self.roots[x] {
            let parent = self.roots[x];
            let root = self.find(parent);
            self.weights[x] *= self.weights[parent];
            self.roots[x] = root;
        }
        self.roots[x]
    }

    pub fn union(&mut self, x: usize, y: usize, value: f64) {
        let (xr, yr) = (self.find(x), self.find(y));
        let (wx, wy) = (self.weights[x], self.weights[y]);
        if xr != yr {
            if self.ranks[xr] > self.ranks[yr] {
                self.roots[yr] = xr;
                self.weights[yr] = wx / (value * wy);
            } else if self.ranks[xr] < self.ranks[yr] {
                self.roots[xr] = yr;
                self.weights[xr] = (value * wy) / wx;
            } else {
                self.roots[yr] = xr;
                self.ranks[xr] += 1;
                self.weights[yr] = wx / (value * wy);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
