use std::collections::HashMap;

// Step 1) create adjacency list of tickets where { from: [to_i]}
// Step 2) sort the list for each entry by lexical value ascending
// Step 3) loop thru the adjacency list for JFK (the entry point)
// Step 4) DFS each to from JFK

fn find_itinerary(tickets: Vec<Vec<String>>) -> Vec<String> {
    let mut adj_list: HashMap<&String, Vec<&String>> = HashMap::new();
    let jfk = String::from("JFK");

    for ticket in tickets.iter() {
        let from = &ticket[0];
        let to = &ticket[1];

        adj_list.entry(from).or_default().push(to);
    }

    for destination in adj_list.values_mut() {
        destination.sort_by(|a, b| b.cmp(a));
    }

    let mut itinerary: Vec<String> = Vec::new();

    dfs(&jfk, &mut adj_list, &mut itinerary);
    itinerary.reverse();
    itinerary
}

fn dfs(
    vertex: &String,
    adj_list: &mut HashMap<&String, Vec<&String>>,
    itinerary: &mut Vec<String>,
) {
    while let Some(node) = adj_list.get_mut(vertex).and_then(|dest| dest.pop()) {
        dfs(node, adj_list, itinerary);
    }
    itinerary.push(vertex.clone());
}
