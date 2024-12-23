use std::collections::{HashMap, HashSet};
use std::hash::Hash;


pub struct BiDirectionalGraph<Key, Node> {
    nodes: HashMap<Key, Node>,
    edges: HashMap<Key, HashSet<Key>>, // edge A-B is present in the set for both A and B
}

impl<Key: Eq + Hash + Clone, Node> BiDirectionalGraph<Key, Node> {
    pub fn new() -> Self {
        BiDirectionalGraph { nodes: HashMap::new(), edges: HashMap::new() }
    }

    pub fn nodes(&self) -> &HashMap<Key, Node> {
        &self.nodes
    }

    pub fn edges(&self) -> &HashMap<Key, HashSet<Key>> {
        &self.edges
    }

    pub fn has_node(&self, key: &Key) -> bool {
        self.nodes.contains_key(key)
    }

    pub fn insert_node(&mut self, key: Key, node: Node) {
        self.nodes.insert(key, node);
    }

    pub fn get_edges(&self, key: &Key) -> &HashSet<Key> {
        &self.edges[key]
    }

    pub fn insert_edge(&mut self, first: Key, second: Key) {
        assert!(self.nodes.contains_key(&first));
        assert!(self.nodes.contains_key(&second));
        self.insert_edge_one_direction(first.clone(), second.clone());
        self.insert_edge_one_direction(second, first);
    }

    fn insert_edge_one_direction(&mut self, first: Key, second: Key) {
        self.edges.entry(first).or_insert_with(|| HashSet::new()).insert(second);
    }
}
