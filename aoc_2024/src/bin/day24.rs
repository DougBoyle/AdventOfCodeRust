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

Three bit errors (11 + 01 = 100):
Input bit 8: errors [9]
Input bit 9: errors [9, 10, 11]
Input bit 11: errors [13, 14]
Input bit 12: errors [13, 14]
Input bit 13: errors [13, 14, 15]
Input bit 17: errors [19, 20]
Input bit 19: errors [19, 20, 21]
Input bit 31: errors [33, 34]
Input bit 32: errors [33, 34]

!! 5 possible errors per bit, how to distinguish?
    XOR / final carry interchangeable

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
    part2_validation_experiment(gates);

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
    test_three_bit_errors(gates);
}

fn test_one_bit_errors(gates: &Gates) {
    let mut sort_gates = TopoSortGates { gates: &gates };
    let gates_order = sort_gates.sort().unwrap();

    println!("Single bit errors:");
    for i in 0..45 {
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
    for i in 0..45 {
        let x = 1 << i;
        let expected = 2*x;
        let actual = test_input(x, x, gates, &gates_order);
        if expected != actual {
            println!("Input bit {i} error:\nExpected: {expected:0>46b}\nActual:   {actual:0>46b}");
        }
    }
    println!("");
}

fn test_three_bit_errors(gates: &Gates) {
    let mut sort_gates = TopoSortGates { gates: &gates };
    let gates_order = sort_gates.sort().unwrap();

    println!("Three bit errors (11 + 01 = 100):");
    for i in 0..44 {
        let x = (1 << i) | (1 << i+1);
        let y = 1 << i;
        let expected = x + y;
        let actual = test_input(x, y, gates, &gates_order);
        if expected != actual {
            println!("Input bit {i} error:\nExpected: {expected:0>46b}\nActual:   {actual:0>46b}");
        }
    }
    println!("");
}

fn test_input(x: u64, y: u64, gates: &Gates, gates_order: &Vec<&Label>) -> u64 {
    let mut outputs = HashMap::new();
    for bit in 0..45 {
        outputs.insert(Label::InputX(bit), ((x >> bit) & 1) != 0);
        outputs.insert(Label::InputY(bit), ((y >> bit) & 1) != 0);
    }

    eval(&mut outputs, &gates, &gates_order)
}

// Only direct/indirect carry appear as inputs to an OR gate (excluding z00, no indirect carry to OR with)
fn is_direct_or_indirect_carry_from_usage(gate: &Label, gates: &Gates) -> bool {
    gates.values()
        .any(|Gate { kind, left, right }| *kind == GateKind::OR && (gate == left || gate == right))
}

// Could incrementally go:
// literal x/y 00 gates -> indirect gates and output
// previous indirect gate + literal x/y 01 gates -> output 01 and 01 indirect gate
// if set every grows more than expected, need to change some wires, but unclear which?
fn part2_validation_experiment(gates: Gates) {
    // 222 gates
    // Except for first/last digit:
    // XOR x/y/carry to get output - 2 new gates
    // (x AND y) can produce a carry, else so can ((x ^ y) AND previous carry), OR these for actual carry - 3 new gates

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

    // Need ways to validate each output, even when other outputs could be wrong (but inputs always correct!):
    // 'xor':
    // x_i ^ y_i => {}
    // Some G s.t.  '{} ^ G => _' and '{} & G => _'  -- can also use to pair up 'final carry i' to 'xor i+1' 
    
    // 'direct carry'
    // x_i & y_i => {}
    // '{} OR _ => _'
    
    // 'indirect carry'
    // '{} OR _ => _'

    // Can split into literal operations 'x_i & y_i' / 'x_i ^ y_i' and derived operations on generated outputs.

    let all_gates: HashSet<_> = gates.iter().flat_map(|(output, Gate { left, right, .. })| [left, right, output]).collect();

    // Direct / Indirect carry gates are interchangeable (for the same bit)
    // Subset of all possible direct/indirect carry gates: Appears in an OR gate - 88 gates, must be one of them!
    let all_possible_direct_indirect_carries: HashSet<_> = all_gates.iter().copied()
        .filter(|gate| is_direct_or_indirect_carry_from_usage(gate, &gates)).collect();
    
    // Interchangeable pairs of gates
    let carry_and_next_xor_pairs: Vec<HashSet<_>> = gates.values()
        .filter(|gate| gate.kind == GateKind::AND) // could equally have taken the XOR gate
        .filter(|Gate { left, right, .. }| !left.is_input() && !right.is_input())
        .map(|Gate { left, right, .. }| [left, right].into_iter().collect::<HashSet<_>>())
        .collect();
    // Subset of all possible xor/final carry gates: Appears in both an XOR and AND - 88 gates, must be one of them!
    let all_possible_xor_and_final_carries: Vec<_> = carry_and_next_xor_pairs.iter().flat_map(|set| set.iter()).collect();
    // Final carry: Of the derived operations: On LHS of XOR / AND, and RHS of OR. Take best 2 of 3?
    
    
    // Outputs are either x00 ^ y00 = z00, 'derived | derived == z45', or for all others 'derived ^ derived = z_i'
    let incorrect_outputs: Vec<_> = (0..46).filter_map(|i| {
        let gate = &gates[&Label::OutputZ(i)];
        let Gate { kind, left, right } = gate;
        let correct = if i == 0 {
            let (a, b) = (Label::InputX(i), Label::InputY(i));
            *kind == GateKind::XOR && (*left == a && *right == b || *left == b && *right == a)
        } else if i == 45 {
            *kind == GateKind::OR && left.is_intermediate_gate() && right.is_intermediate_gate()
        } else {
            *kind == GateKind::XOR && left.is_intermediate_gate() && right.is_intermediate_gate()
        };
        if correct { None } else { Some((i, gate)) }
    }).collect();
    println!("{} incorrect output gates: {incorrect_outputs:?}", incorrect_outputs.len());

    // x_n ^ y_n -> xor gate != z00
    let mut xor_gates = BTreeMap::new();
    for (output, Gate { left, right, kind }) in &gates {
        if *kind != GateKind::XOR { continue; }
        if let Some(bit) = matching_input_bits(left, right) {
            xor_gates.insert(bit, output);
        }
    }
    println!("Found {} of 45 XOR gates: {xor_gates:?}", xor_gates.len());
    assert_eq!(45, xor_gates.len());
    // On the RHS of an XOR, but not used as one (could also be a final carry, not detected here)
    let incorrect_xor_outputs: Vec<_> = xor_gates.iter()
        .filter(|(_, gate)| ***gate != Label::OutputZ(0) && !all_possible_xor_and_final_carries.contains(gate))
        .collect();
    // 1 - nnt - appears in an OR - must be a direct/indirect carry instead
    println!("Found {} incorrect XOR outputs: {incorrect_xor_outputs:?}", incorrect_xor_outputs.len());

    let mut direct_carry_gates = BTreeMap::new();
    for (output, Gate { left, right, kind }) in &gates {
        if *kind != GateKind::AND { continue; }
        if let Some(bit) = matching_input_bits(left, right) {
            direct_carry_gates.insert(bit, output);
        }
    }
    println!("Found {} of 45 direct carry gates: {direct_carry_gates:?}", direct_carry_gates.len());
    assert_eq!(45, direct_carry_gates.len());

    // On the RHS of a literal AND, but not used in later OR (excluding x00 & y00)
    let incorrect_direct_carries: Vec<_> = direct_carry_gates.iter()
        .filter(|(&i, gate)| i != 0 && !all_possible_direct_indirect_carries.contains(*gate))
        .collect();
    // 1 - nnt - appears in an OR - must be a direct/indirect carry instead
    println!("Found {} incorrect direct carry outputs: {incorrect_direct_carries:?}", incorrect_direct_carries.len());

    let mut indirect_carry_gates = BTreeMap::new();
    let mut final_carry_gates = BTreeMap::new();
    // No indirect carry for the first bit, so this is done specially
    final_carry_gates.insert(0, direct_carry_gates[&0]);
    for i in 1..44 {
        let xor = xor_gates[&i];
        let direct_carry = direct_carry_gates[&i];
        let previous_carry = match final_carry_gates.get(&(i - 1)) {
            Some(previous_carry) => *previous_carry,
            None => continue,
        };

        if let Some(indirect_carry) = gates.iter().filter(|(_, gate)|
            gate.kind == GateKind::AND
            && (gate.left == *xor && gate.right == *previous_carry 
                || gate.left == *previous_carry && gate.right == *xor)
        ).map(|(output, _)| output).next() {
            indirect_carry_gates.insert(i, indirect_carry);

            // 'direct carry i OR indirect carry i -> final carry 01'
            gates.iter().filter(|(_, gate)|
                gate.kind == GateKind::OR
                && (gate.left == *direct_carry && gate.right == *indirect_carry 
                    || gate.left == *indirect_carry && gate.right == *direct_carry)
            ).map(|(output, _)| output).next()
            .and_then(|final_carry| final_carry_gates.insert(i, final_carry));
        }
        if !final_carry_gates.contains_key(&i) {
            // Instead try to determine final carry based on 'xor i+1 ^ final carry i -> z i+1'
            let next_xor = xor_gates[&(i + 1)];
            let next_output = Label::OutputZ(i + 1);
            gates.iter().filter_map(|(output, gate)| {
                if gate.kind != GateKind::XOR || *output != next_output {
                    None
                } else if gate.left == *next_xor {
                    Some(&gate.right)
                } else if gate.right == *next_xor {
                    Some(&gate.left)
                } else {
                    None
                }
            }).next().and_then(|final_carry| final_carry_gates.insert(i, final_carry));
        }
    }

    println!("Found {} of 44 indirect carry gates: {indirect_carry_gates:?}", indirect_carry_gates.len());
 //   assert_eq!(44, indirect_carry_gates.len());

    println!("Found {} of 45 final carry gates: {final_carry_gates:?}", final_carry_gates.len());
 //   assert_eq!(45, final_carry_gates.len());
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
