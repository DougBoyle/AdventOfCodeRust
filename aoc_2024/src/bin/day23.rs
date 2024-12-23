use std::collections::HashSet;
use std::hash::Hash;

use aoc_2024::read_input;
use rust_aoc::{graph::BiDirectionalGraph, split_in_two};

fn main() {
    part1();
    part2();
}

fn part1() {
    let graph = parse_input();

    let triples = find_triples(&graph).iter()
        .filter(|nodes| nodes.iter().any(|node| node.starts_with("t")))
        .count();

    println!("Total {}", triples); // 1184
}

// TODO: Maximal Clique Problem: https://en.wikipedia.org/wiki/Bron%E2%80%93Kerbosch_algorithm
fn part2() {
    let graph = parse_input();
    /* 
    println!("Nodes: {}, Edges: {}", graph.nodes().len(), graph.edges().values().map(|set| set.len()).sum::<usize>());

    let triples = find_triples(&graph);

    println!("3: {}", triples.len());

    let mut cliques = triples;
    let mut password = None;
    for i in 4..16 {
        cliques = cliques.into_iter().flat_map(|clique| extend_clique(clique, &graph).into_iter()).collect();
        println!("{i}: {}", cliques.len());
        if cliques.is_empty() {
            break;
        } else if cliques.len() == 1 {
            password = Some(get_password(&cliques[0]));
        } else {
            password = None;
        }
    }
    */
    let mut max_size = 0;
    let mut password = None;
    bron_kerbosch_max_cliques(|clique| {
        if clique.len() > max_size {
            max_size = clique.len();
            password = Some(get_password(clique));
        } else if clique.len() == max_size {
            password = None; // clear password on ties
        }
    }, &graph);

    println!("Password: {}", password.unwrap()); // hf,hz,lb,lm,ls,my,ps,qu,ra,uc,vi,xz,yv
}

fn get_password(clique: &Vec<&String>) -> String {
    let mut copy: Vec<_> = clique.iter().map(|s| String::from(*s)).collect();
    copy.sort();
    copy.join(",")
}

fn find_triples(graph: &Graph) -> Vec<Vec<&String>> {
    let mut triples = Vec::new();

    for first_node in graph.nodes().keys() {
        let first_node_edges = graph.get_edges(first_node);
        for second_node in first_node_edges.iter().filter(|node| *node > first_node) {
            for third_node in first_node_edges.iter()
            .filter(|node| *node > second_node && graph.get_edges(second_node).contains(*node)) {
                let mut triple = vec![first_node, second_node, third_node];
                triple.sort();
                triples.push(triple);
            }
        }
    }

    triples
}

/// Given a clique of size N, return all cliques of size N+1 where the original clique is a
/// subset, and the new node is "greater" than all added so far (to avoid reporting duplicates).
/// It is important that we return all, a Clique of 3 (A, B, C) may also be fully connected
/// to node D, as well as nodes E and F, but if D is not connected to E and F then it might
/// not be part of the maximal clique and will be filtered out at the next size.
#[allow(dead_code)]
fn extend_clique<'a>(clique: Vec<&'a String>, graph: &'a Graph) -> Vec<Vec<&'a String>> {
    let max_node = *clique.iter().max().unwrap();
    let each_new_nodes = clique.iter()
        .map(|node| graph.get_edges(node))
        .map(|node_edges| node_edges.iter().filter(|node| *node > max_node).collect());
    let new_nodes = intersect_all(each_new_nodes).unwrap();
    new_nodes.into_iter().map(|node| {
        let mut new_clique = clique.clone();
        new_clique.push(node);
        new_clique
    }).collect()
}

/// https://en.wikipedia.org/wiki/Bron%E2%80%93Kerbosch_algorithm
/// Reports all maximal cliques - fully connected groups that cannot be extended, not just the largest such clique.
/// 
/// Relies on 3 sets (called R/P/X in the online description):
/// Clique - Initially empty, current set of fully connected nodes found, can potentially be extended.
/// Candidates - Initially all nodes of the graph, nodes connected to every node in Clique to consider adding.
/// Excluded - Initially empty, nodes connected to all nodes of the graph, but excluded by prior searches.
/// 
/// (Candidates U Excluded) is all nodes connected to every node of Clique, but we filter out already explored
/// nodes to avoid reporting the same clique many times.
/// When (Candidates U Excluded) is empty, this is a maximal clique.
/// When just Candidates is empty, we backtrack, as the only larger cliques we can form are ones already explored.
/// 
/// On its own, does a lot of backtracking for each non-maximal clique, where Candidates is empty but Excluded is not.
/// To avoid this, pick any "pivot" node from (Candidates U Excluded), and skip recusing on any of its neighbours in
/// Candidates. To see why, consider a maximal clique that would have been found from searching one of its neighbours:
/// - If the clique includes the pivot node, we would also find it when searching the pivot itself.
/// - If the clique does not include the pivot, it must include at least one non-neighbour of the pivot. If it did not,
///   the pivot was chosen from (Candidates U Excluded) so is connected to everything already in Clique, and to all of
///   its neighbours, so this new clique cannot be maximal since the pivot could be added to it.
/// The best choice of pivot is the one with most neighbours in Candidates, minimising the number of recursive calls.
fn bron_kerbosch_max_cliques<F: FnMut(&Vec<&String>)>(mut f: F, graph: &Graph) {
    let nodes = graph.nodes().keys().collect();
    bron_kerbosch_max_cliques_helper(&mut Vec::new(), nodes, HashSet::new(), &mut f, graph);
}

fn bron_kerbosch_max_cliques_helper<'a, F: FnMut(&Vec<&String>)>(
    clique: &mut Vec<&'a String>,
    mut candidates: HashSet<&'a String>,
    mut excluded: HashSet<&'a String>,
    f: &mut F,
    graph: &'a Graph
) {
    if candidates.is_empty() && excluded.is_empty() {
        f(clique);
        return;
    }
    // Choose pivot and get its relevant edges to exclude
    let skip_neighbours = candidates.union(&excluded)
        .map(|pivot| candidates.intersection(&graph.get_edges(pivot).iter().collect()).map(|s| *s).collect::<HashSet<_>>())
        .max_by_key(|neighbours| neighbours.len())
        .unwrap();
    // Iterate remaining candidates
    let to_explore = candidates.iter().map(|s| *s).filter(|s| !skip_neighbours.contains(s)).collect::<Vec<_>>();
    for candidate in to_explore {
        clique.push(candidate);
        let edges = graph.get_edges(candidate).iter().collect();
        let new_candidates = candidates.intersection(&edges).map(|s| *s).collect();
        let new_excluded = excluded.intersection(&edges).map(|s| *s).collect();
        bron_kerbosch_max_cliques_helper(clique, new_candidates, new_excluded, f, graph);

        candidates.remove(candidate);
        excluded.insert(candidate);
        clique.pop();
    }
}

fn intersect_all<T: Eq + Hash + Copy>(sets: impl Iterator<Item=HashSet<T>>) -> Option<HashSet<T>> {
    sets.reduce(|set1, set2| set1.intersection(&set2).map(|s| *s).collect())
}

// TODO: Replace with int keys and String values?
type Graph = BiDirectionalGraph<String, ()>;

fn parse_input() -> Graph {
    let mut graph = Graph::new();
    for line in read_input(23) {
        let (first, second) = split_in_two(&line, '-');
        let first = first.to_string();
        let second = second.to_string();
        if !graph.has_node(&first) { graph.insert_node(first.clone(), ()); }
        if !graph.has_node(&second) { graph.insert_node(second.clone(), ()); }
        graph.insert_edge(first, second);
    }
    graph
}
