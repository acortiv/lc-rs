use std::collections::{HashMap, HashSet, VecDeque};

pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
    if !word_list.contains(&end_word) {
        return 0;
    }

    let n = begin_word.len();

    // Build the graph
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();
    for word in &word_list {
        for i in 0..n {
            // wildcard root
            let mut w_word: Vec<char> = word.clone().chars().collect();
            w_word[i] = '*';
            let w_word: String = w_word.iter().collect();
            graph.entry(w_word).or_insert(Vec::new()).push(word.clone());
        }
    }

    let mut q = VecDeque::new();

    let mut visited = HashSet::new();
    visited.insert(begin_word.clone());

    q.push_back((begin_word, 1));

    while let Some((word, dist)) = q.pop_front() {
        if word == end_word {
            return dist;
        }

        for i in 0..n {
            let mut w_word: Vec<char> = word.chars().collect();
            w_word[i] = '*';
            let w_word: String = w_word.iter().collect();
            if let Some(v_words) = graph.get(&w_word) {
                for word in v_words {
                    if visited.insert(word.clone()) {
                        q.push_back((word.clone(), dist + 1))
                    }
                }
            }
        }
    }

    0
}

// TODO: Bye optimization
