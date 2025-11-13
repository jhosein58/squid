use std::collections::{HashMap, VecDeque};

use squid_core::AudioNode;

type PortId = usize;
type NodeId = usize;

#[derive(Debug, Clone)]
pub struct Edge {
    pub from_node: NodeId,
    pub from_port: PortId,
    pub to_node: NodeId,
    pub to_port: PortId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NodeColor {
    White,
    Gray,
    Black,
}

pub struct AudioGraph {
    pub nodes: HashMap<NodeId, Box<dyn AudioNode>>,
    next_id: NodeId,
    pub edges: Vec<Edge>,
    processing_order: Vec<NodeId>,
}

impl AudioGraph {
    pub fn new() -> Self {
        AudioGraph {
            nodes: HashMap::new(),
            next_id: 0,
            edges: Vec::new(),
            processing_order: Vec::new(),
        }
    }

    pub fn add_node(&mut self, node: Box<dyn AudioNode>) -> NodeId {
        let id = self.next_id;
        self.nodes.insert(id, node);
        self.next_id += 1;
        id
    }

    pub fn add_connection(
        &mut self,
        from_node: NodeId,
        from_port: PortId,
        to_node: NodeId,
        to_port: PortId,
    ) {
        self.edges.push(Edge {
            from_node,
            from_port,
            to_node,
            to_port,
        });
    }

    pub fn detect_feedback_edges<'a>(&'a self) -> Vec<&'a Edge> {
        let mut feedback_edges = Vec::new();
        let mut colors: HashMap<NodeId, NodeColor> = self
            .nodes
            .keys()
            .map(|id| (*id, NodeColor::White))
            .collect();

        for node_id in self.nodes.keys() {
            if colors.get(node_id) == Some(&NodeColor::White) {
                self.dfs_visit(*node_id, &mut colors, &mut feedback_edges);
            }
        }

        feedback_edges
    }

    fn dfs_visit<'a>(
        &'a self,
        u: NodeId,
        colors: &mut HashMap<NodeId, NodeColor>,
        feedback_edges: &mut Vec<&'a Edge>,
    ) {
        colors.insert(u, NodeColor::Gray);

        for edge in self.edges.iter().filter(|e| e.from_node == u) {
            let v = edge.to_node;

            match colors.get(&v) {
                Some(&NodeColor::Gray) => {
                    feedback_edges.push(edge);
                }
                Some(&NodeColor::White) => {
                    self.dfs_visit(v, colors, feedback_edges);
                }
                Some(&NodeColor::Black) => {}
                None => {}
            }
        }
        colors.insert(u, NodeColor::Black);
    }

    pub fn rebuild_processing_order(&mut self) {
        let feedback_edges = self.detect_feedback_edges();

        let mut in_degrees: HashMap<NodeId, usize> = self.nodes.keys().map(|&id| (id, 0)).collect();

        for edge in &self.edges {
            let is_feedback = feedback_edges.iter().any(|fe| std::ptr::eq(*fe, edge));
            if !is_feedback {
                if let Some(degree) = in_degrees.get_mut(&edge.to_node) {
                    *degree += 1;
                }
            }
        }

        // --- FIX STARTS HERE ---
        // 1. Collect nodes with in-degree 0 into a Vec
        let mut initial_nodes: Vec<NodeId> = in_degrees
            .iter()
            .filter(|&(_, &degree)| degree == 0)
            .map(|(&id, _)| id)
            .collect();

        // 2. Sort the initial nodes to ensure deterministic behavior
        initial_nodes.sort_unstable(); // Sorting by NodeId is efficient and deterministic

        // 3. Create the queue from the sorted Vec
        let mut queue: VecDeque<NodeId> = initial_nodes.into();
        // --- FIX ENDS HERE ---

        let mut order = Vec::with_capacity(self.nodes.len());

        while let Some(u) = queue.pop_front() {
            order.push(u);

            for edge in self.edges.iter().filter(|e| e.from_node == u) {
                let is_feedback = feedback_edges.iter().any(|fe| std::ptr::eq(*fe, edge));
                if !is_feedback {
                    let v = edge.to_node;
                    if let Some(degree) = in_degrees.get_mut(&v) {
                        *degree -= 1;
                        if *degree == 0 {
                            queue.push_back(v);
                        }
                    }
                }
            }
        }

        // To make the entire process deterministic, you could sort neighbors before adding them to the queue,
        // but sorting the initial queue is usually enough to fix such bugs.
        // The main loop logic itself seems correct.

        self.processing_order = order;
    }

    pub fn get_processing_order(&self) -> &[NodeId] {
        &self.processing_order
    }
}
