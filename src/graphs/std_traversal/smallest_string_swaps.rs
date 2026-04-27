use ::std::collections::HashMap;

pub fn smallest_string_with_swaps(s: String, pairs: Vec<Vec<i32>>) -> String {
    let n = s.len();
    let mut uf = UnionFind::new(n);

    // Build the graph
    for edge in pairs.iter() {
        let u = edge[0] as usize;
        let v = edge[1] as usize;
        uf.union(u, v);
    }

    let chars: Vec<char> = s.chars().collect();

    // Group indices by connected component root
    let mut groups: HashMap<usize, Vec<usize>> = HashMap::new();
    for i in 0..n {
        let root = uf.find(i);
        groups.entry(root).or_default().push(i);
    }

    // Build results in place
    let mut result = chars.clone();
    for indices in groups.values_mut() {
        let mut component_chars: Vec<char> = indices.iter().map(|&i| chars[i]).collect();
        indices.sort_unstable();
        component_chars.sort_unstable();
        for (&idx, &ch) in indices.iter().zip(component_chars.iter()) {
            result[idx] = ch;
        }
    }

    result.into_iter().collect()
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
        let (xr, yr) = (self.find(x), self.find(y));
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn uf_swap() {
        let s = String::from("dcab");
        let pairs = vec![vec![0, 3], vec![1, 2], vec![0, 2]];
        assert_eq!(smallest_string_with_swaps(s, pairs), "abcd")
    }
}
