use std::{collections::{HashMap, HashSet}, hash::Hash};

use bimap::BiMap;

pub type Key = u32;

pub struct BiDirectionalGraph<Node> {
    nodes: BiMap<Key, Node>,
    edges: HashMap<Key, HashSet<Key>>, // edge A-B is present in the set for both A and B
    next_key: Key,
}

impl<Node: Eq + Hash> BiDirectionalGraph<Node> {
    pub fn new() -> Self {
        BiDirectionalGraph { nodes: BiMap::new(), edges: HashMap::new(), next_key: 0 }
    }

    pub fn nodes(&self) -> impl Iterator<Item=&Node> {
        self.nodes.right_values()
    }

    pub fn keys(&self) -> impl Iterator<Item=Key> + '_ {
        self.nodes.left_values().copied()
    }

    pub fn edges(&self) -> &HashMap<Key, HashSet<Key>> {
        &self.edges
    }

    pub fn get_key(&self, node: &Node) -> Option<Key> {
        self.nodes.get_by_right(node).copied()
    }

    pub fn get_key_or_insert(&mut self, node: Node) -> Key {
        self.nodes.get_by_right(&node).copied().unwrap_or_else(|| {
            self.insert_node(node)
        })
    }

    pub fn get_node(&self, key: &Key) -> Option<&Node> {
        self.nodes.get_by_left(key)
    }

    pub fn insert_node(&mut self, node: Node) -> Key {
        let key = self.next_key;
        self.next_key += 1;
        self.nodes.insert(key, node);
        key
    }

    pub fn get_edges(&self, key: &Key) -> impl Iterator<Item=Key> + '_ {
        self.edges[key].iter().copied()
    }

    pub fn has_edge(&self, first: &Key, second: &Key) -> bool {
        self.edges.get(first).is_some_and(|edges| edges.contains(second))
    }

    pub fn insert_edge(&mut self, first: Key, second: Key) {
        assert!(self.nodes.contains_left(&first));
        assert!(self.nodes.contains_left(&second));
        self.insert_edge_one_direction(first.clone(), second.clone());
        self.insert_edge_one_direction(second, first);
    }

    fn insert_edge_one_direction(&mut self, first: Key, second: Key) {
        self.edges.entry(first).or_insert_with(|| HashSet::new()).insert(second);
    }
}
