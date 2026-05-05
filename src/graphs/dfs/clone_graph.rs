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
    let _visited = vec![false; n];
    

    Graph {
        vertices: n,
        edges: n - 1,
        nodes: vec![],
    }
}

fn dfs() -> () {
    unimplemented!()
}
