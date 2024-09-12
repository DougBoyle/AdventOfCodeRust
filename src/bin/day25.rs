use std::{collections::{HashMap, HashSet}, rc::Rc, time::Instant};

use bimap::BiMap;
use priority_queue::PriorityQueue;


fn main() {
    part1();
}

fn part1() {
    let graph = Graph::load();

    let len = graph.node_ids.len();
    println!("Length is {len}");

    let start = Instant::now();
    // Version 1: ~300ms
    // Version 2: Separate HashSet of String node IDs, and edges instead just store Rc<String> references: No change
    // Version 3: Index of ID -> details of node, reduces time to ~230ms
    let (merged_graph, half_of_cut_id) = find_min_cut_stoer_wagner(&graph);
    println!("Stoer Wagner took t={}ms",(Instant::now() - start).as_millis()); 

    let len1 = merged_graph.get_node_size(&half_of_cut_id);
    let len2 = len - len1;

    println!("Part 1: Split into {len1} and {len2}, result = {}", len1 * len2); // 583632
}

const CUTS_ALLOWED: usize = 3;

/*
See https://en.wikipedia.org/wiki/Stoer%E2%80%93Wagner_algorithm 
or https://citeseerx.ist.psu.edu/document?repid=rep1&type=pdf&doi=b10145f7fc3d07e43607abc2a148e58d24ced543

High-level idea:
- Given two vertices s and t, the minimum cut is either a minimum s-t cut (separating s and t),
  or the same as a minimum cut of the graph after merging those two nodes.
- Hence to find a minimum cut, we just need to be able to find an arbitrary s and t node and
  their minimum s-t cut, and take the lesser of that and repeating on the graph with s and t merged.
- We can find a minimum s-t cut by building up a set of nodes in a particular order, such that
  s and t are the last two nodes we add, and separating off just the last node t forms a minimum s-t cut
  (with weight being the weight of all edges out of t).
- To do that, start from any node 'a' and build a set A by repeatedly adding the most 'tightly connected'
  node i.e. the node with greatest total weight of its edges to nodes in A:
  x s.t. w(A, x) >= w(A, y) for all y not in A, where w(A, x) is weight of edges from A to x.

Proof: 
  Say we picked nodes in the order A = {a, ..., s, t} and consider an arbitrary s-t cut C.
  We say v != a is 'active' if v and the previous node are on either sides of C.
  (like induction on times we cross sides of C, but not quite, given v and the previous node may not be adjacent)
  Let w(C) = weight of edges of cut C, A_v is the nodes of A up to but not including v, C_v is the cut of subset
  A_v U {v} caused by the edges of C (which exists since v and the previous node are on opposite sides of C).

  Lemma: For each active node v: w(A_v, v) <= w(C_v)
  Base case, first active node v:
    First active node, so all of A_v on same side of C, hence w(C_v) = w(A_v, v), just the edges between A_v to v.
  Assume up to active node v, then consider next active node u:
    w(A_u, u) = w(A_v, u) + w(A_u / A_v, u)
    - we previously chose v because it was the most tightly connected to A_v, so w(A_v, v) >= w(A_v, u)
    w(A_u, u) <= w(A_v, v) + w(A_u / A_v, u)
    - induction
    w(A_u, u) <= w(C_v) + w(A_u / A_v, u)
    - The edges counted in w(A_u / A_v, u) are all edges to u, and u is not in A_v U {v}, so none are already counted in w(C_v).
      (A_u / A_v) = {v, ...} are all nodes on the same side as v, and opposite side to u, since u was the next active node, 
      hence every edge counted in w(A_u / A_v, u) is an edge across sides of C and part of w(C_u)
    w(A_u, u) <= w(C_v) + w(A_u / A_v, u) <= w(C_u)

  As C is an s-t cut, the last node t is always active, and C_t = V (all nodes), so w(V / {t}, t) <= w(C).
  Hence the cut that just removes the last node t is the minimum s-t cut for last two nodes s, t.
 */
fn find_min_cut_stoer_wagner(graph: &Graph) -> (Graph, usize) {
    let mut graph = graph.clone();
    while graph.edges.len() > 1 {
        let (s, t, weight) = minimum_cut_phase(&graph);
        if weight <= CUTS_ALLOWED {
            // expand merged node t into the actual partition
            return (graph, t)
        } else {
            graph.merge_nodes(&s, &t);
        }
    }
    panic!("Didn't find a suitable cut");
}

fn minimum_cut_phase(graph: &Graph) -> (NodeId, NodeId, usize) {
    let mut queue: PriorityQueue<NodeId, usize> = PriorityQueue::new();
    for node in graph.edges.keys() {
        queue.push(*node, 0);
    }
    let mut found = vec![];
    let mut last_weight = 0;

    while queue.len() > 0 {
        let (node, cut_weight) = remove_node_and_update_weights(&mut queue, &graph);
        found.push(node);
        last_weight = cut_weight;
    }

    let mut it = found.into_iter().rev();
    let last = it.next().unwrap();
    let second_last = it.next().unwrap();
    (second_last, last, last_weight)
}

fn remove_node_and_update_weights(queue: &mut PriorityQueue<NodeId, usize>, graph: &Graph) -> (NodeId, usize) {
    let (node, cut_weight) = queue.pop().unwrap();
    for (neighbour, edge_weight) in &graph.edges[&node] {
        queue.change_priority_by(neighbour, |p| *p += edge_weight);
    }
    (node, cut_weight)
}

#[derive(Clone)]
struct Graph {
    node_ids: BiMap<Node, NodeId>,
    edges: HashMap<NodeId, HashMap<NodeId, usize>>,
}

type NodeId = usize;

// Every node has an ID, and is either a Simple node with a String name, or composite node referencing 2 other IDs.
// Having an 'index' avoids string copying, or needing to use Rc eveywhere.
#[derive(Clone, Eq, PartialEq, Hash)]
enum Node {
    Simple(String),
    Composite(NodeId, NodeId)
}

impl Graph {
    fn load() -> Graph {
        let mut graph = Graph { node_ids: BiMap::new(), edges: HashMap::new() };
        for line in rust_aoc::read_input(25) {
            let (node, neighbours) = rust_aoc::split_in_two(&line, ':');
            let (node, neighbours) = (node.trim(), neighbours.trim());
            let node = graph.get_or_create_simple_node(String::from(node));
            for neighbour in neighbours.split_ascii_whitespace() {
                let neighbour = graph.get_or_create_simple_node(String::from(neighbour));
                graph.add_or_update_edge(&node, &neighbour, 1);
            }
        }
        graph
    }

    fn merge_nodes(&mut self, s: &NodeId, t: &NodeId) {
        let (s, s_neighbours) = self.edges.remove_entry(s).unwrap();
        let (t, t_neighbours) = self.edges.remove_entry(t).unwrap();
    
        let new_node = self.create_composite_node(s, t);
    
        for (neighbour, _) in s_neighbours {
            if neighbour == t { continue; }
            let neighbour_edges = self.edges.get_mut(&neighbour).unwrap();
            if let Some(weight) = neighbour_edges.remove(&s) {
                self.add_or_update_edge(&new_node, &neighbour, weight);
            }
        }
        for (neighbour, _) in t_neighbours {
            if neighbour == s { continue; }
            let neighbour_edges = self.edges.get_mut(&neighbour).unwrap();
            if let Some(weight) = neighbour_edges.remove(&t) {
                self.add_or_update_edge(&new_node, &neighbour, weight);
            }
        }
    }

    // from and to must already exist as nodes in the graph
    fn add_or_update_edge(&mut self, from: &NodeId, to: &NodeId, weight: usize) {
        // edges are bi-directional, but only reported in one direction
        self.edges.get_mut(from).unwrap().entry(*to)
            .and_modify(|w| *w += weight).or_insert(weight);
        self.edges.get_mut(to).unwrap().entry(*from)
            .and_modify(|w| *w += weight).or_insert(weight);
    }

    fn get_or_create_simple_node(&mut self, node: String) -> NodeId {
        let node = Node::Simple(node);
        match self.node_ids.get_by_left(&node) {
            Some(node) => *node,
            None => {
                let id = self.get_next_id();
                self.insert_node(node, id);
                id
            }
        }
    }

    fn create_composite_node(&mut self, first: NodeId, second: NodeId) -> NodeId {
        let id = self.get_next_id();
        let node = Node::Composite(first, second);
        self.insert_node(node, id);
        id
    }

    fn insert_node(&mut self, node: Node, id: NodeId) {
        self.node_ids.insert(node, id);
        self.edges.insert(id, HashMap::new());
    }

    fn get_next_id(&self) -> NodeId {
        self.node_ids.len()
    }

    fn get_node_size(&self, id: &NodeId) -> usize {
        match self.node_ids.get_by_right(id).unwrap() {
            Node::Simple(_) => 1,
            // We want to number of original leaves, the Composite node itself doesn't count towards size
            Node::Composite(left, right) => self.get_node_size(left) + self.get_node_size(right),
        }
    }
}
