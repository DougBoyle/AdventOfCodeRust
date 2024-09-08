use core::panic;
use std::{borrow::{Borrow, BorrowMut}, cell::RefCell, collections::{HashMap, HashSet, VecDeque}, fmt::{Debug, Display}, io::{Error, ErrorKind}, rc::Rc, str::FromStr, time::Instant};

fn main() {
    part1();
    part2();
    part2_alt();
}

fn part1() {
    let mut machine = Machine::load();
    let mut low_total = 0;
    let mut high_total = 0;
    for _ in 0..1000 {
        let (low, high) = machine.press();
        low_total += low;
        high_total += high;
    }
    println!("Low {low_total}, High {high_total}, Product {}", low_total * high_total); // 763500168
}

fn part2() {
    let mut machine = Machine::load();

    machine.print_adjacency();

    let graph = Machine::parse();

    // try to simplify by walking back
    let rx = &graph["rx"]; // want to see it receive low
    let to_rx = &rx.input_modules;
    println!("{to_rx:?} feeds into {}", rx.id);

    let to_rx: Vec<_> = to_rx.iter().map(|id| &graph[id]).collect();
    println!("{} feeds into {rx}", to_rx.iter().map(|node| node.to_string()).collect::<Vec<_>>().join(", "));

    if to_rx.len() != 1 || to_rx[0].module_type != ModuleType::Conjunction {
        panic!("Expected one Conjunction input to rx");
    }

    // dh -> rx, want to see dh emit a low signal => everything feeding into dh emits high
    let dh = to_rx[0];
    let to_dh = &dh.input_modules;
    println!("{to_dh:?} feeds into {}", dh.id);

    let to_dh: Vec<_> = to_dh.iter().map(|id| &graph[id]).collect();
    println!("L2 {} feeds into {dh}", to_dh.iter().map(|node| node.to_string()).collect::<Vec<_>>().join(", "));

    if to_dh.len() != 4 || to_dh.iter().any(|module| module.module_type != ModuleType::Conjunction) {
        panic!("Expected 4 Conjunction inputs to dh");
    }


    // tr, xm, dr, nh -> dh, all Conjuntions.
    // Want all of those to emit high => at least one node before each of them emits low
    let mut all_l3s = vec![];
    for l2 in to_dh {
        let l3s = &l2.input_modules;
        println!("{l3s:?} feeds into {}", l2.id);
    
        let mut l3s: Vec<_> = l3s.iter().map(|id| &graph[id]).collect();
        println!("{} feeds into {l2}", l3s.iter().map(|node| node.to_string()).collect::<Vec<_>>().join(", "));
        if l3s.len() != 1 { panic!("Only expected one input to each l2 node"); }
        all_l3s.append(&mut l3s);
    }
    
    if all_l3s.len() != 4 || all_l3s.iter().any(|module| module.module_type != ModuleType::Conjunction) {
        panic!("Expected 4 Conjunction inputs at level 3 (-> level 2 -> dh -> rx)");
    }

    // vc, tn, hd, jx
    println!("L3 Conjunctions (all need to emit low): {}", all_l3s.iter().map(|node| node.to_string()).collect::<Vec<_>>().join(", "));

    let mut all_l4s = HashSet::new();
    for l3 in all_l3s {
        let l4s = &l3.input_modules;
        println!("{l4s:?} feeds into {}", l3.id);
    
        let l4s: Vec<_> = l4s.iter().map(|id| &graph[id]).collect();
        println!("{} feeds into {l3}", l4s.iter().map(|node| node.to_string()).collect::<Vec<_>>().join(", "));

        if l4s.iter().any(|l4| all_l4s.contains(l4) || l4.module_type != ModuleType::FlipFlop) {
            panic!("Expected inputs to L3 conjunctions to all be disjoint flip-flops");
        }
        for l4 in l4s {
            all_l4s.insert(l4);
        }
    }

    // L4 flip flops all high -> L3 Conjunctions all low -> L2 conjunctions all high -> dh low -> rx
    println!("L4 FlipFlops (all need to emit high): {}", all_l4s.iter().map(|node| node.to_string()).collect::<Vec<_>>().join(", "));

    // Record whenever those modules emit High
    for l4 in &all_l4s {
        (*machine.modules[&l4.id]).borrow_mut().get_base_module_mut().track_high_output = true;
    }

    let num_presses = 1_000_000;

    let mut i = 0;
    let start = Instant::now();
    while i < num_presses {
        i += 1;
        machine.press();
        if i % 1_000_000 == 0 { println!("Press {}m, t={}s", i/1_000_000, (Instant::now() - start).as_secs()); }
    }
    println!("Complete after {i} presses");

    for l4 in &all_l4s {
        let module = (*machine.modules[&l4.id]).borrow();
        let base_mod = module.get_base_module();
        let same_cycle = base_mod.high_press_and_cycle.iter().map(|(_, cycle)| cycle).collect::<HashSet<_>>().len() == 1;
        if !same_cycle { panic!("{} publishes High output on inconsistent cycle", base_mod.id); }
    }

    // Attempt to find cycle for each case. Gaps between High outputs might differ, so consider a window of
    // up to OUTPUTS_TO_COLLECT / 4, and verify it against the rest of the gaps recorded.
    let periods: Vec<u64> = all_l4s.iter().map(|l4| {
        let module = (*machine.modules[&l4.id]).borrow();
        let base_mod = module.get_base_module();
        let deltas = base_mod.high_press_and_cycle.iter()
            .map(|(press, _)| press)
            .collect::<Vec<_>>()
            .windows(2)
            .map(|window| window[1] - window[0])
            .collect::<Vec<_>>();

        let deltas_period = (1..deltas.len()/4).filter(|deltas_period| {
            let reference_deltas = &deltas[..*deltas_period];

            (1..deltas.len()/deltas_period).all(|window| (0..*deltas_period).all(|i| {
                deltas[window*deltas_period + i] == reference_deltas[i]
            }))
        }).next().expect(&format!("Could not find period of gaps for {} after {} presses, deltas={:?}", base_mod.id, num_presses, deltas));

        let period = base_mod.high_press_and_cycle[deltas_period].0 - base_mod.high_press_and_cycle[0].0;

        println!("{} has deltas period {} -> period {}", base_mod.id, deltas_period, period);

        period as u64
    }).collect();

    println!("Periods: {periods:?}");
    println!("LCM: {}", periods.iter().map(|n| *n as u64).reduce(|n, m| num::integer::lcm(n, m)).unwrap()); // 207652583562007
}

fn part2_alt() {
    let mut machine = Machine::load();

    // machine.print_adjacency();

    let graph = Machine::parse();

    // try to simplify by walking back
    let rx = &graph["rx"]; // want to see it receive low
    let to_rx = &rx.input_modules;
    println!("{to_rx:?} feeds into {}", rx.id);

    let to_rx: Vec<_> = to_rx.iter().map(|id| &graph[id]).collect();
    println!("{} feeds into {rx}", to_rx.iter().map(|node| node.to_string()).collect::<Vec<_>>().join(", "));

    if to_rx.len() != 1 || to_rx[0].module_type != ModuleType::Conjunction {
        panic!("Expected one Conjunction input to rx");
    }

    // dh -> rx, want to see dh emit a low signal => everything feeding into dh emits high
    let dh = to_rx[0];
    let to_dh = &dh.input_modules;
    println!("{to_dh:?} feeds into {}", dh.id);

    let to_dh: Vec<_> = to_dh.iter().map(|id| &graph[id]).collect();
    println!("L2 {} feeds into {dh}", to_dh.iter().map(|node| node.to_string()).collect::<Vec<_>>().join(", "));

    if to_dh.len() != 4 || to_dh.iter().any(|module| module.module_type != ModuleType::Conjunction) {
        panic!("Expected 4 Conjunction inputs to dh");
    }


    // tr, xm, dr, nh -> dh, all Conjuntions.
    // Want all of those to emit high => at least one node before each of them emits low

    // Record whenever those modules emit High
    for l2 in &to_dh {
        (*machine.modules[&l2.id]).borrow_mut().get_base_module_mut().track_high_output = true;
    }

    let mut i = 0;
    let start = Instant::now();
    while !machine.all_outputs_registered(1) {
        i += 1;
        machine.press();
        if i % 1_000_000 == 0 { println!("Press {}m, t={}s", i/1_000_000, (Instant::now() - start).as_secs()); }
    }
    println!("Complete after {i} presses");

    let cycles: Vec<_> = to_dh.iter().map(|l2| {
        let module = (*machine.modules[&l2.id]).borrow();
        let base_mod = module.get_base_module();
        println!("{} - {:?}", base_mod.id, base_mod.high_press_and_cycle);
        base_mod.high_press_and_cycle[0].0
    }).collect();
    println!("Cycles: {:?}", cycles);
    println!("LCM: {}", cycles.iter().map(|n| *n as u64).reduce(|n, m| num::integer::lcm(n, m)).unwrap()); // 207652583562007
}


// Just unrolled directly in Machine::press
// const BUTTON: &str = "button";
const BROADCASTER: &str = "broadcaster";
const OUTPUTS_TO_COLLECT: usize = 10_000;

// Reference: Release build: 5M loops in 34s, 10M loops in 69s
// 1. Update Pulse to store raw pointers to IDs, allowing never cloning values.
// Result: 5M in 12s, 10M in 23s -- over 2x faster. 50M loops in 121s, 140M loops in 338s.
// 2. Put pointer to sender in queue, rather than putting 1 entry in queue for each destination.
// Result: 50M in 83s, 140M in 236s -- ~30% faster again. 200M in 338s, still no result after 830M in 1230s.
// 3. Use Rc to point to other nodes directly, rather than going via HashMap, 
//    and RefCell to do updates on 'immutable' instances rather than needing unsafe raw mut pointers.
// Result: 50M in 31s, 200M in 125s -- ~2.5x faster
// 4. Rc and RefCell everywhere to get rid of all uses of unsafe:
// Result: 50M in 44s, 200M in 175s -- ~30% slower, but still much faster than version using HashMaps
struct Machine {
    modules: HashMap<String, ModuleRef>,
    presses: usize,
}

impl Machine {
    fn parse() -> HashMap<String, ParsedModule> {
        let mut parsed_modules: HashMap<String, RefCell<ParsedModule>> = rust_aoc::read_input(20)
            .map(|s| s.parse().unwrap())
            .map(|module: ParsedModule| (module.id.clone(), RefCell::new(module)))
            .collect();
        let output_module = ParsedModule::new(String::from("rx"), ModuleType::Receiver, vec![]);
        parsed_modules.insert(output_module.id.clone(), RefCell::new(output_module));
        // join up inputs for each node, needed to implement Conjunction
        for module in parsed_modules.values() {
            let module = module.borrow();
            for child in &module.downstream_modules {
                parsed_modules[child].borrow_mut().add_input(module.id.clone());
            }
        }
        parsed_modules.into_iter().map(|(id, cell)| (id.clone(), cell.into_inner())).collect()
    }

    fn load() -> Machine {
        let parsed_modules = Machine::parse();

        let modules: HashMap<String, ModuleRef> = parsed_modules.iter()
            .map(|(id, parsed)| (id.clone(), parsed.create_module()))
            .collect();
        // join up actual modules
        for from in parsed_modules.values() {
            let from_ref = &modules[&from.id];
            let mut from_module = (**from_ref).borrow_mut();
            for to in &from.downstream_modules {
                let to_ref = &modules[to];
                let mut to_module = (**to_ref).borrow_mut();
                from_module.get_base_module_mut().add_output(&to_ref);
                to_module.get_base_module_mut().add_input(&from_ref);
            }
        }

        Machine { modules, presses: 0 }
    }

    fn press(&mut self) -> (usize, usize) {
        self.presses += 1;
        let mut low_count = 1;
        let mut high_count = 0;
        let mut queue: VecDeque<Pulse> = VecDeque::new();

        let broadcaster = &self.modules[BROADCASTER];
        queue.push_back(Pulse { sender: Rc::clone(broadcaster), pulse_type: PulseType::Low, cycle: 1 });
        while let Some(Pulse { sender, pulse_type, cycle }) = queue.pop_front() {
            let next_cycle = cycle+1;
            let sender = (*sender).borrow();
            let sender_base = sender.get_base_module();

            match pulse_type {
                PulseType::High => high_count += sender_base.downstream_modules.len(),
                PulseType::Low => low_count += sender_base.downstream_modules.len(),
            }

            for module in &sender_base.downstream_modules {
                let output_pulse = (**module).borrow_mut().process(&sender_base.id, pulse_type);
                if let Some(output_pulse) = output_pulse {
                    (**module).borrow_mut().get_base_module_mut().track_outputs(output_pulse, self.presses, next_cycle);
                    queue.push_back(Pulse {
                        sender: Rc::clone(module), 
                        pulse_type: output_pulse,
                        cycle: next_cycle,
                    });
                }
            }
        }

        (low_count, high_count)
    }

    fn received_low(&self) -> bool {
        (*self.modules["rx"]).borrow().received_low()
    }

    fn all_outputs_registered(&self, count: usize) -> bool {
        self.modules.values().all(|module| {
            let module = (**module).borrow();
            let base_mod = module.get_base_module();
            base_mod.track_high_output == false || base_mod.high_press_and_cycle.len() >= count
        })
    }

    fn print_adjacency(&self) {
        for (id, module) in &self.modules {
            let downstream: Vec<String> = (**module).borrow().get_base_module().downstream_modules.iter()
                .map(|module| (**module).borrow().get_base_module().id.clone())
                .collect();
            if downstream.is_empty() {
                println!("{id}");
            } else {
                let downstream = downstream.join(";");
                println!("{id};{downstream}");
            }
        }
    }
}

#[derive(Clone)]
struct ParsedModule {
    id: String,
    module_type: ModuleType,
    input_modules: Vec<String>,
    downstream_modules: Vec<String>,
}

impl ParsedModule {
    fn new(id: String, module_type: ModuleType, downstream_modules: Vec<String>) -> ParsedModule {
        ParsedModule { id, module_type, input_modules: vec![], downstream_modules }
    }

    fn add_input(&mut self, id: String) {
        self.input_modules.push(id);
    }

    fn create_module(&self) -> Rc<RefCell<dyn Module>> {
        let base_module = BaseModule::new(self.id.clone(), self.module_type);
        match base_module.module_type {
            ModuleType::Broadcast => Rc::new(RefCell::new(BroadcastModule::new(base_module, self))),
            ModuleType::Conjunction => Rc::new(RefCell::new(ConjunctionModule::new(base_module, self))),
            ModuleType::FlipFlop => Rc::new(RefCell::new(FlipFlopModule::new(base_module, self))),
            ModuleType::Receiver => Rc::new(RefCell::new(ReceiverModule::new(base_module, self))),
        }
    }
}

impl Eq for ParsedModule {}

impl PartialEq for ParsedModule {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl std::hash::Hash for ParsedModule {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state)
    }
}

impl Display for ParsedModule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{} ({:?})", self.id, self.module_type))
    }
}

impl FromStr for ParsedModule {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (id, downstream_modules) = s.split_once(" -> ").unwrap();
        let downstream_modules = downstream_modules.split(", ").map(String::from).collect();
        if id == "broadcaster" {
            Ok(ParsedModule::new(String::from(id), ModuleType::Broadcast, downstream_modules))
        } else if id.starts_with('%') {
            Ok(ParsedModule::new(String::from(&id[1..]), ModuleType::FlipFlop, downstream_modules))
        } else if id.starts_with('&') {
            Ok(ParsedModule::new(String::from(&id[1..]), ModuleType::Conjunction, downstream_modules))
        } else {
            Err(Error::new(ErrorKind::InvalidInput, format!("Unrecognised ID {id}")))
        }
    }
}

type ModuleRef = Rc<RefCell<dyn Module>>;

struct BaseModule {
    id: String,
    module_type: ModuleType,
    input_modules: Vec<ModuleRef>,
    downstream_modules: Vec<ModuleRef>, // TODO: Creates a cycle, don't worry about it for now
    track_high_output: bool,
    high_press_and_cycle: Vec<(usize, usize)>,
}

impl BaseModule {
    fn new(id: String, module_type: ModuleType) -> BaseModule {
        BaseModule { id, module_type, input_modules: vec![], downstream_modules: vec![], track_high_output: false, high_press_and_cycle: vec![] }
    }

    fn add_input(&mut self, input: &ModuleRef) {
        self.input_modules.push(Rc::clone(input));
    }

    fn add_output(&mut self, output: &ModuleRef) {
        self.downstream_modules.push(Rc::clone(output));
    }

    fn track_outputs(&mut self, output: PulseType, press: usize, cycle: usize) {
        if self.track_high_output && output == PulseType::High && self.high_press_and_cycle.len() < OUTPUTS_TO_COLLECT {
            self.high_press_and_cycle.push((press, cycle));
        }
    }
}

trait Module {
    fn new(base: BaseModule, parsed: &ParsedModule) -> Self where Self: Sized;
    fn process(&mut self, from: &str, pulse_type: PulseType) -> Option<PulseType>;
    fn get_base_module(&self) -> &BaseModule;
    fn get_base_module_mut(&mut self) -> &mut BaseModule;
    
    fn received_low(&self) -> bool {
        false
    }
}

struct BroadcastModule {
    base: BaseModule,
}

impl Module for BroadcastModule {
    fn new(base: BaseModule, _parsed: &ParsedModule) -> Self {
        BroadcastModule { base }
    }

    fn process(&mut self, _from: &str, pulse_type: PulseType) -> Option<PulseType> {
        Some(pulse_type)
    }

    fn get_base_module(&self) -> &BaseModule {
        &self.base
    }

    fn get_base_module_mut(&mut self) -> &mut BaseModule {
        &mut self.base
    }
}

struct ConjunctionModule {
    base: BaseModule,
    inputs: HashMap<String, PulseType>,
}

impl Module for ConjunctionModule {
    fn new(base: BaseModule, parsed: &ParsedModule) -> Self {
        let inputs = parsed.input_modules.iter().map(|module| (module.clone(), PulseType::Low)).collect();
        ConjunctionModule { base, inputs }
    }

    fn process(&mut self, from: &str, pulse_type: PulseType) -> Option<PulseType> {
        *self.inputs.borrow_mut().get_mut(from).unwrap() = pulse_type;
        if self.inputs.borrow().values().all(|input| *input == PulseType::High) {
            Some(PulseType::Low)
        } else {
            Some(PulseType::High)
        }
    }

    fn get_base_module(&self) -> &BaseModule {
        &self.base
    }

    fn get_base_module_mut(&mut self) -> &mut BaseModule {
        &mut self.base
    }
}

struct FlipFlopModule {
    base: BaseModule,
    on: bool,
}

impl Module for FlipFlopModule {
    fn new(base: BaseModule, _parsed: &ParsedModule) -> Self {
        FlipFlopModule { base, on: false }
    }

    fn process(&mut self, _from: &str, pulse_type: PulseType) -> Option<PulseType> {
        match pulse_type {
            PulseType::High => None,
            PulseType::Low  => {
                let mut on = self.on.borrow_mut();
                *on = !*on;
                Some(if *on { PulseType::High } else { PulseType::Low })
            }
        }
    }

    fn get_base_module(&self) -> &BaseModule {
        &self.base
    }

    fn get_base_module_mut(&mut self) -> &mut BaseModule {
        &mut self.base
    }
}

struct ReceiverModule {
    base: BaseModule,
    received_low: RefCell<bool>,
}

impl Module for ReceiverModule {
    fn new(base: BaseModule, _parsed: &ParsedModule) -> Self {
        ReceiverModule { base, received_low: RefCell::new(false) }
    }

    fn process(&mut self, _from: &str, pulse_type: PulseType) -> Option<PulseType> {
        if pulse_type == PulseType::Low { *self.received_low.borrow_mut() = true }
        None
    }

    fn get_base_module(&self) -> &BaseModule {
        &self.base
    }

    fn get_base_module_mut(&mut self) -> &mut BaseModule {
        &mut self.base
    }

    fn received_low(&self) -> bool {
        *self.received_low.borrow()
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum ModuleType { Broadcast, Conjunction, FlipFlop, Receiver }

struct Pulse {
    sender: ModuleRef,
    pulse_type: PulseType,
    cycle: usize,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum PulseType { High, Low }
