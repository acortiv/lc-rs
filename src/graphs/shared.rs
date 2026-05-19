use std::{cell::RefCell, rc::Rc};

#[derive(Debug, PartialEq, Eq)]
pub struct Node {
    pub val: i32,
    pub left: Option<Rc<RefCell<Node>>>,
    pub right: Option<Rc<RefCell<Node>>>,
    pub next: Option<Rc<RefCell<Node>>>,
}

impl Node {
    pub fn new(val: i32) -> Self {
        Self {
            val,
            left: None,
            right: None,
            next: None,
        }
    }
}

// TODO: Figure out a common Graph/Node representation for leetcode problems (or a couple common ones,
// that go beyond the standard Vec<Vec<usize>> adjacency list.) This is a problem I need to solve,
// for some of the legacy leetcode problems.

/* pub struct Node {
    pub val: i32,
    pub left: Option<Rc<RefCell<Node>>>,
    pub right: Option<Rc<RefCell<Node>>>,
}

impl Node {
    pub fn new(
        value: Option<i32>,
        left: Option<Rc<RefCell<Node>>>,
        right: Option<Rc<RefCell<Node>>>,
    ) -> Self {
        match (value, left, right) {
            (Some(val), Some(l_node), Some(r_node)) => {
                return Self {
                    val,
                    left: Some(l_node),
                    right: Some(r_node),
                };
            }
            (Some(val), None, None) => {
                return Self {
                    val,
                    left: None,
                    right: None,
                };
            }
            _ => {
                return Self {
                    val: 0,
                    left: None,
                    right: None,
                };
            }
        }
    }
}

pub struct Graph {
    pub nodes: Option<Vec<Rc<RefCell<Option<Node>>>>>,
}

impl Graph {
    pub fn new(nodes: Option<Vec<Node>>) -> Self {
        if let Some(nodes) = nodes {
            return Self {
                nodes: Some(Vec::from(
                    nodes
                        .into_iter()
                        .map(|node| Rc::new(RefCell::new(Some(node))))
                        .collect::<Vec<Rc<RefCell<Option<Node>>>>>(),
                )),
            };
        }

        Self { nodes: None }
    }
} */
