// Question:
// Given the edges of a directed graph where edges[i] = [ai, bi] indicates there is an edge between nodes ai and bi,
// and two nodes source and destination of this graph, determine whether or not all paths starting from source eventually, end at destination, that is:

// At least one path exists from the source node to the destination node
// If a path exists from the source node to a node with no outgoing edges, then that node is equal to destination.
// The number of possible paths from source to destination is a finite number.

// Return true if and only if all roads from source lead to destination.
// *********************************************************************
// Rules:
// There if there is a cycle, it is not possible to reach the destination from that path and therefore fails
// If there are outbound connections from the destination, fails
// *********************************************************************

pub fn leads_to_destination(n: i32, edges: Vec<Vec<i32>>, source: i32, destination: i32) -> bool {
    let n = n as usize;
    let source = source as usize;
    let destination = destination as usize;

    // Color the nodes of the graph, where:
    // 0: not visited
    // 1: not safe
    // 2: safe
    let mut state: Vec<u8> = vec![0; n];

    // standard adjacency list using Vec<Vec<usize>> to record edges
    let mut adj_list: Vec<Vec<usize>> = vec![Vec::new(); n];

    for edge in edges.iter() {
        let u = edge[0] as usize;
        let v = edge[1] as usize;

        adj_list[u].push(v);
    }

    dfs(source, destination, &adj_list, &mut state)
}

fn dfs(node: usize, destination: usize, edges: &[Vec<usize>], state: &mut Vec<u8>) -> bool {
    // Checks needed:
    // 1) If we reached destination, are there outbonud connections for the destination: if yes, return false, otherwise return true
    // 2) Have we reached an unsafe node, if yes, return false

    match state[node] {
        1 => return false,
        2 => return true,
        _ => {}
    }

    match node == destination {
        true => return edges[node].is_empty(),
        _ if edges[node].is_empty() => return false,
        _ => {}
    }

    state[node] = 1;
    for &edge in edges[node].iter() {
        if !dfs(edge, destination, edges, state) {
            return false;
        }
    }

    state[node] = 2;
    true
}
