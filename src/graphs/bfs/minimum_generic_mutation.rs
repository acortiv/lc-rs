// what state do i need to model
// node shape: (gene_string, working_idx, mutation_count)
// where gene_string: Vec<char>

use std::collections::{HashSet, VecDeque};

pub fn min_mutation(start_gene: String, end_gene: String, bank: Vec<String>) -> i32 {
    let bank: HashSet<String> = bank.into_iter().collect();

    if !bank.contains(&end_gene) {
        return -1;
    }

    let genes = ['A', 'C', 'G', 'T'];

    let mut visited = HashSet::new();
    visited.insert(start_gene.clone());

    let mut q = VecDeque::new();
    q.push_back((start_gene, 0));

    while let Some((gene_seq, dist)) = q.pop_front() {
        if gene_seq == end_gene {
            return dist;
        }

        let mut chars: Vec<char> = gene_seq.chars().collect();

        for i in 0..chars.len() {
            let original = chars[i];

            for &c in &genes {
                if c == original {
                    continue;
                }

                chars[i] = c;
                let next: String = chars.iter().collect();
                if bank.contains(&next) && visited.insert(next.clone()) {
                    q.push_back((next, dist + 1));
                }
            }

            chars[i] = original;
        }
    }

    -1
}
