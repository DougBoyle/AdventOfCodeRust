use std::collections::{BTreeMap, HashMap, HashSet};
use std::fmt::Debug;
use std::hash::Hash;
use std::str::FromStr;

use aoc_2024::read_input;
use rust_aoc::TopologicalSort;
use rust_aoc::split_in_two;

fn main() {
    part1();
    part2();
}

const MAX_INPUT: u32 = 44;
const MAX_OUTPUT: u32 = MAX_INPUT + 1;

fn part1() {
    let Input { mut outputs, gates } = parse_input();

    let mut sort_gates = TopoSortGates { gates: &gates };
    let gates_order = sort_gates.sort().unwrap();
    let total = eval(&mut outputs, &gates, &gates_order);

    println!("Total {total}"); // 42049478636360
}

/*
Note: Can't immediately categorise the different failures from this alone.

Single bit errors:
Input bit 9: errors [9, 10]     -- solved by swapping NNT and GWS (Direct carry / XOR for bit 9)
Input bit 13: errors [13, 14]   -- solved by swapping Z13 and NPF (Output / final carry for bit 13)
Input bit 19: errors [19, 20]   -- solved by swapping Z19 and CPH (Output / direct carry for bit 19)
Input bit 33: errors [33, 34]   -- solved by swapping Z33 and HGJ (Output / indirect carry for bit 33)

Two bit errors:
Input bit 9: errors [9, 10]     -- solved by swapping NNT and GWS (Direct carry / XOR for bit 9)
Input bit 12: errors [13, 14]   -- solved by swapping Z13 and NPF (Output / final carry for bit 13)
Input bit 13: errors [13, 14]   -- solved by swapping Z13 and NPF (Output / final carry for bit 13)
Input bit 18: errors [19, 20]   -- solved by swapping Z19 and CPH (Output / direct carry for bit 19)
Input bit 19: errors [19, 20]   -- solved by swapping Z19 and CPH (Output / direct carry for bit 19)
Input bit 32: errors [33, 34]   -- solved by swapping Z33 and HGJ (Output / indirect carry for bit 33)
*/
fn part2() {
    let Input { mut gates, .. } = parse_input();

    println!("Num gates: {}", gates.len());

    let swaps: Vec<(Label, Label)> = vec![];// vec![("nnt", "gws"), ("z19", "cph"), ("z13", "npf"), ("z33", "hgj")];
    for (first, second) in &swaps {
        let first_gate = gates.remove(first).unwrap();
        let second_gate = gates.remove(second).unwrap();
        gates.insert(first.clone(), second_gate);
        gates.insert(second.clone(), first_gate);
    }

    part2_output_validation(&gates);
    part2_validation_experiment(&gates);

    // Another idea, add topological sort edges and look for swaps to resolve that (more generic problem):
    // Obviously need x/y values before same z value, and should be able to compute z_i without later x/y values.
    // x_i, y_i -> z_i -> x_(i+1), y_(i+1)

    let mut answer: Vec<_> = swaps.into_iter().flat_map(|(a, b)| [a, b]).map(|label| label.to_string()).collect();
    answer.sort();
    let answer = answer.join(",");
    println!("Answer: {answer}"); // cph,gws,hgj,nnt,npf,z13,z19,z33
}

fn part2_output_validation(gates: &Gates) {
    test_one_bit_errors(gates);
    test_two_bit_errors(gates);
}

fn test_one_bit_errors(gates: &Gates) {
    let mut sort_gates = TopoSortGates { gates: &gates };
    let gates_order = sort_gates.sort().unwrap();

    println!("Single bit errors:");
    for i in 0..MAX_OUTPUT {
        let x = 1 << i;
        let actual = test_input(x, 0, gates, &gates_order);
        if x != actual {
            println!("Input bit {i} error:\nExpected: {x:0>46b}\nActual:   {actual:0>46b}");
        }
    }
    println!("");
}

fn test_two_bit_errors(gates: &Gates) {
    let mut sort_gates = TopoSortGates { gates: &gates };
    let gates_order = sort_gates.sort().unwrap();

    println!("Two bit errors:");
    for i in 0..MAX_OUTPUT {
        let x = 1 << i;
        let expected = 2*x;
        let actual = test_input(x, x, gates, &gates_order);
        if expected != actual {
            println!("Input bit {i} error:\nExpected: {expected:0>46b}\nActual:   {actual:0>46b}");
        }
    }
    println!("");
}

fn test_input(x: u64, y: u64, gates: &Gates, gates_order: &Vec<&Label>) -> u64 {
    let mut outputs = HashMap::new();
    for bit in 0..MAX_OUTPUT {
        outputs.insert(Label::InputX(bit), ((x >> bit) & 1) != 0);
        outputs.insert(Label::InputY(bit), ((y >> bit) & 1) != 0);
    }

    eval(&mut outputs, &gates, &gates_order)
}

/// Basic outline of an adder block, ignoring special cases for first/last bit:
/// x_n ^ y_n -> xor n
/// x_n & y_n -> direct carry n
/// xor n ^ final carry (n-1) -> z_n
/// xor n & final carry (n-1) -> indirect carry n
/// direct carry n OR indirect carry n -> final carry n

/// Only direct/indirect carry appear as inputs to an OR gate (excluding z00, no indirect carry to OR with).
/// Interchangeable for a given bit.
fn is_direct_or_indirect_carry_from_usage(gate: &Label, gates: &Gates) -> bool {
    gates.values()
        .any(|Gate { kind, left, right }| *kind == GateKind::OR && (gate == left || gate == right))
}

/// These pairs are &'d to get next direct carry, and XOR'd to get next output. Interchangeable for a given bit.
fn get_final_carry_and_next_xor_pairs_from_usage(gates: &Gates) -> Vec<HashSet<Label>> {
    gates.values()
        .filter(|gate| gate.kind == GateKind::AND) // could equally have taken the XOR gate
        .filter(|Gate { left, right, .. }| !left.is_input() && !right.is_input()) // distinguish from other AND uses
        .map(|Gate { left, right, .. }| [left, right].into_iter().cloned().collect::<HashSet<_>>())
        .collect()
}

/// x_n ^ y_n -> ? (should be the intermediate gate labelled above as 'xor n', but might not be used like that in later inputs)
fn get_output_wires_of_input_xor_gates(gates: &Gates) -> BTreeMap<u32, Label> {
    gates.iter().filter_map(|(label, Gate { left, right, kind })| {
        if *kind != GateKind::XOR { return None; }
        let n = matching_input_bits(left, right)?;
        Some((n, label.clone()))
    }).collect()
}

/// x_n & y_n -> ? (should be the intermediate gate labelled above as 'direct carry', but might not be used like that in later inputs)
fn get_output_wires_of_input_direct_carry_gates(gates: &Gates) -> BTreeMap<u32, Label> {
    gates.iter().filter_map(|(label, Gate { left, right, kind })| {
        if *kind != GateKind::AND { return None; }
        let n = matching_input_bits(left, right)?;
        Some((n, label.clone()))
    }).collect()
}

fn check_output_gates(gates: &Gates) {
    // z_i gates in the wrong place:
    let misplaced_outputs: Vec<_> = gates.iter()
        .filter(|(label, Gate { kind, left, right })| {
            let correct = match label {
                Label::OutputZ(0) => {
                    let (a, b) = (Label::InputX(0), Label::InputY(0));
                    *kind == GateKind::XOR && (*left == a && *right == b || *left == b && *right == a)
                },
                Label::OutputZ(MAX_OUTPUT) => {
                    *kind == GateKind::OR && left.is_intermediate_gate() && right.is_intermediate_gate()
                },
                Label::OutputZ(_) => {
                    *kind == GateKind::XOR && left.is_intermediate_gate() && right.is_intermediate_gate()
                },
                _ => true,
            };
            !correct
        }).collect();
    println!("Misplaced output wires: {misplaced_outputs:#?}");

    // other gates where z_i expected:
    let other_labels_where_output_expected: Vec<_> = gates.iter()
        .filter(|(label, Gate { kind, left, right })| {
            !label.is_output() && *kind == GateKind::XOR && left.is_intermediate_gate() && right.is_intermediate_gate()
        }).collect();
    println!("Non-output wire after what looks like output gate: {other_labels_where_output_expected:#?}");
}

fn part2_validation_experiment(gates: &Gates) {
    // 222 gates

    // X and Y each 45 bit numbers
    // x00 ^ y00 -> z00
    // x00 & y00 -> direct/final carry 00 (tss)

    // x01 ^ y01 -> xor 01 (rvp)
    // x01 & y01 -> direct carry 01 (jcr)
    // xor 01 (rvp) ^ final carry 00 (tss) -> z01
    // xor 01 (rvp) & final carry 00 (tss) -> indirect carry 01 (bcr)
    // direct carry 01 (jcr) OR indirect carry 01 (bcr) -> final carry 01 (tdp)
    
    // ...

    // x44 ^ y44 -> xor 44 (tsc)
    // x44 & y44 -> direct carry 44 (gnn)
    // xor 44 (tsc) ^ final carry 43 -> z44
    // xor 44 (tsc) & final carry 43 -> indirect carry 44 (kbb)
    // direct carry 44 (gnn) OR indirect carry 44 (kbb) -> z45

    // 2 + 44*5 = 222, no room for any extra gates
    assert_eq!(222, gates.len());

    let all_gates: HashSet<_> = gates.iter().flat_map(|(output, Gate { left, right, .. })| [left, right, output]).collect();

    // Direct / Indirect carry gates are interchangeable (for the same bit)
    // Subset of all possible direct/indirect carry gates
    let all_possible_direct_indirect_carries: HashSet<_> = all_gates.iter().copied()
        .filter(|gate| is_direct_or_indirect_carry_from_usage(gate, gates)).collect();
    
    // Interchangeable pairs of gates
    let carry_and_next_xor_pairs: Vec<HashSet<_>> = get_final_carry_and_next_xor_pairs_from_usage(gates);
    let all_possible_xor_and_final_carries: Vec<_> = carry_and_next_xor_pairs.iter().flat_map(|set| set.iter()).collect();

    check_output_gates(gates);

    let xor_output_wires = get_output_wires_of_input_xor_gates(gates);
    // Output wires of input XORs, but not in the set of XOR and final carry labels as determined by their usage as inputs
    let other_labels_where_xor_expected: BTreeMap<_,_> = xor_output_wires.into_iter()
        .filter(|(i, label)| *i != 0 || *label != Label::OutputZ(0)) // special-case at start
        .filter(|(_, label)| !all_possible_xor_and_final_carries.contains(&label)).collect();
    println!("Non-XOR wire (based on usage) after what looks like inputs XOR gate: {other_labels_where_xor_expected:#?}");

    let direct_carry_output_wires = get_output_wires_of_input_direct_carry_gates(gates);
    // Output wires of 'x_n & y_n', but not in the set of direct/indirect carry labels as determined by their usage as inputs
    let other_labels_where_direct_carry_expected: BTreeMap<_,_> = direct_carry_output_wires.into_iter()
        .filter(|(i, _)| *i != 0) // special-case at start
        .filter(|(_, label)| !all_possible_direct_indirect_carries.contains(label)).collect();
    println!("Non direct carry wire (based on usage) after what looks like x_n & y_n gate: {other_labels_where_direct_carry_expected:#?}");
}

fn get_x_gate(i: u32) -> String {
    format_gate('x', i)
}

fn get_y_gate(i: u32) -> String {
    format_gate('y', i)
}

fn get_output_gate(i: u32) -> String {
    format_gate('z', i)
}

fn format_gate(letter: char, n: u32) -> String {
    format!("{letter}{n:0>2}")
}

fn matching_input_bits(left: &Label, right: &Label) -> Option<u32> {
    match (left, right) {
        (Label::InputX(n), Label::InputY(m)) if n == m => Some(*n),
        (Label::InputY(n), Label::InputX(m)) if n == m => Some(*n),
        _ => None,
    }
}

fn eval(outputs: &mut Outputs, gates: &Gates, gates_order: &Vec<&Label>) -> u64 {
    for &wire in gates_order {
        // sorting includes the direct wire values listed as gate inputs, so skip those
        if !wire.is_input() {
            outputs.insert(wire.clone(), gates[wire].eval(&outputs));
        }
    }

    let mut total: u64 = 0;
    for i in 0..46 {
        if outputs[&Label::OutputZ(i)] {
            total += 1 << i;
        }
    }
    total
}

struct TopoSortGates<'a> {
    gates: &'a Gates,
}

impl<'a> TopologicalSort for TopoSortGates<'a> {
    type Node = Label;

    fn get_all_nodes(&self) -> Vec<&Self::Node> {
        self.gates.keys().collect()
    }

    fn get_edges(&self, node: &Self::Node) -> Vec<&Self::Node> {
        match self.gates.get(node) {
            None => vec![],
            Some(Gate { left, right, .. }) => vec![left, right],
        }
    }

    fn edges_point_to_dependents() -> bool {
        false // edges point to the earlier wires required to calculate a gate's output
    }
}

type Gates = HashMap<Label, Gate>;
type Outputs = HashMap<Label, bool>;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum GateKind {
    AND, OR, XOR
}

#[derive(Clone, Eq, PartialEq, Hash)]
enum Label {
    InputX(u32),
    InputY(u32),
    OutputZ(u32),
    Intermediate(String),
}

impl Label {
    fn is_input(&self) -> bool {
        match self {
            Label::InputX(_) | Label::InputY(_) => true,
            _ => false,
        }
    }

    fn is_output(&self) -> bool {
        matches!(self, Label::OutputZ(_))
    }

    fn is_intermediate_gate(&self) -> bool {
        matches!(self, Label::Intermediate(_))
    }
}

impl ToString for Label {
    fn to_string(&self) -> String {
        match self {
            Label::InputX(n) => get_x_gate(*n),
            Label::InputY(n) => get_y_gate(*n),
            Label::OutputZ(n) => get_output_gate(*n),
            Label::Intermediate(s) => s.clone(),
        }
    }
}

impl FromStr for Label {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.chars().next() {
            Some('x') => Label::InputX(s[1..].parse()?),
            Some('y') => Label::InputY(s[1..].parse()?),
            Some('z') => Label::OutputZ(s[1..].parse()?),
            _ => Label::Intermediate(s.to_string()),
        })
    }
}

impl Debug for Label {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[derive(Debug)]
struct Gate {
    kind: GateKind,
    left: Label,
    right: Label,
}

impl Gate {
    fn eval(&self, outputs: &Outputs) -> bool {
        let left = outputs[&self.left];
        let right = outputs[&self.right];
        match self.kind {
            GateKind::AND => left & right,
            GateKind::OR => left | right,
            GateKind::XOR => left ^ right,
        }
    }
}

struct Input {
    outputs: Outputs,
    gates: Gates,
}

fn parse_input() -> Input {
    let mut lines = read_input(24).map(|s| s.trim().to_string());
    let outputs = lines.by_ref().take_while(|s| !s.is_empty()).map(|s| parse_wire_state(&s)).collect();
    let gates: Gates = lines.map(|s| parse_gate(&s)).collect();

    Input { outputs, gates }
}

fn parse_wire_state(line: &str) -> (Label, bool) {
    let (wire, state) = split_in_two(line, ':');
    let wire = wire.parse().unwrap();
    let state = match state.trim() {
        "0" => false,
        "1" => true,
        _ => panic!("Unexpected state '{state}'"),
    };
    (wire, state)
}

fn parse_gate(line: &str) -> (Label, Gate) {
    let [gate, output_wire] = line.split(" -> ").collect::<Vec<_>>().try_into().unwrap();
    let [left, op, right] = gate.split_ascii_whitespace().collect::<Vec<_>>().try_into().unwrap();
    let kind = match op {
        "AND" => GateKind::AND,
        "OR" => GateKind::OR,
        "XOR" => GateKind::XOR,
        _ => panic!("Unexpected operation '{op}'"),
    };
    (output_wire.parse().unwrap(), Gate { kind, left: left.parse().unwrap(), right: right.parse().unwrap() })
}
