use std::{cell::RefCell, rc::Rc};

struct Graph {
    vertices: usize,
    edges: usize,
    /// this will have the strong reference to each individual node
    nodes: Vec<Rc<RefCell<Node>>>,
}

struct Node {
    val: u8,
    /// in my head, these should all be weak references
    neighbors: Vec<Rc<RefCell<Node>>>,
}

fn clone_graph(nodes: Vec<Vec<Node>>) -> Graph {
    let n = nodes.len();
    let mut visited = vec![false; n];
    let mut graph = Graph {
        vertices: n,
        edges: n - 1,
        nodes: vec![],
    };

    graph
}

fn dfs() -> () {
    unimplemented!()
}
