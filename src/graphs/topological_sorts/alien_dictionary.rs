use std::collections::{HashMap, VecDeque};

// Original
pub fn alien_order_dirty(words: Vec<String>) -> String {
    let n = words.len() - 1;
    // Need an adjacency_list, in-degree list, map of "char": node_num, and res list (off the top of my head), and words split by chars (Vec<Vec<char>>)
    let mut char_to_node: HashMap<char, usize> = HashMap::new();
    let mut res: Vec<char> = Vec::new();

    let words: Vec<Vec<char>> = words
        .into_iter()
        .map(|word| word.chars().collect())
        .collect();

    for word in words.iter() {
        for &ch in word.iter() {
            let id = char_to_node.len();
            char_to_node.entry(ch).or_insert(id);
        }
    }

    let m = char_to_node.len();
    let mut adj_list: Vec<Vec<usize>> = vec![Vec::new(); m];
    let mut in_degree: Vec<i32> = vec![0; m];

    // Next, we want to use two pointers to compare the current node to its neighbor (char0 in string a, char1 in string b)
    // Exit early: string1 > string2 && string1.len() sized slice of string0 == string 1
    // Build graph and compute in-degrees of each node matched to it's corresponding character
    for i in 0..n {
        let j = i + 1;
        let (s_zed_len, s_prime_len) = (words[i].len(), words[j].len());
        if s_zed_len != s_prime_len {
            let packed = (s_zed_len, s_prime_len);
            match packed {
                packed if packed.0 > packed.1 => {
                    let (_, s_prime_len) = packed;
                    if words[i][0..s_prime_len] == words[j][..] {
                        return "".into();
                    }
                }
                _ => {}
            }
        }

        for (&char1, &char2) in words[i].iter().zip(words[j].iter()) {
            if char1 != char2 {
                adj_list[*char_to_node.get(&char1).unwrap()]
                    .push(*char_to_node.get(&char2).unwrap());
                in_degree[*char_to_node.get(&char2).unwrap()] += 1;
                break;
            }
        }
    }

    let mut q: VecDeque<usize> = in_degree
        .iter()
        .enumerate()
        .map(|(idx, &val)| if val == 0 { idx } else { usize::MAX / 2 })
        .filter(|&val| val != usize::MAX / 2)
        .collect();

    if q.is_empty() {
        return "".into();
    }

    let node_to_char: HashMap<usize, char> = char_to_node
        .into_iter()
        .map(|(key, value)| (value, key))
        .collect();

    while let Some(node) = q.pop_front() {
        let &char = node_to_char.get(&node).unwrap();
        res.push(char);
        for &dependent in adj_list[node].iter() {
            in_degree[dependent] -= 1;
            if in_degree[dependent] == 0 {
                q.push_back(dependent);
            }
        }
    }

    if res.len() != m {
        "".to_string()
    } else {
        res.into_iter().collect()
    }
}

// cleaned up
pub fn alien_order(words: Vec<String>) -> String {
    let words: Vec<Vec<char>> = words
        .into_iter()
        .map(|word| word.chars().collect())
        .collect();

    let mut char_to_node = HashMap::new();

    for word in &words {
        for &ch in word {
            let id = char_to_node.len();
            char_to_node.entry(ch).or_insert(id);
        }
    }

    let n = char_to_node.len();
    let mut graph = vec![Vec::new(); n];
    let mut indegree = vec![0; n];

    for pair in words.windows(2) {
        let (a, b) = (&pair[0], &pair[1]);

        if a.len() > b.len() && a.starts_with(b) {
            return String::new();
        }

        if let Some((&from, &to)) = a.iter().zip(b).find(|(x, y)| x != y) {
            let u = char_to_node[&from];
            let v = char_to_node[&to];

            graph[u].push(v);
            indegree[v] += 1;
        }
    }

    let node_to_char: Vec<char> = {
        let mut chars = vec!['\0'; n];

        for (&ch, &id) in &char_to_node {
            chars[id] = ch;
        }

        chars
    };

    let mut q: VecDeque<usize> = indegree
        .iter()
        .enumerate()
        .filter_map(|(id, &deg)| (deg == 0).then_some(id))
        .collect();

    let mut result = String::new();

    while let Some(node) = q.pop_front() {
        result.push(node_to_char[node]);

        for &next in &graph[node] {
            indegree[next] -= 1;

            if indegree[next] == 0 {
                q.push_back(next);
            }
        }
    }

    if result.len() == n {
        result
    } else {
        String::new()
    }
}
