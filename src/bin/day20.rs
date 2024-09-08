use std::{collections::{HashMap, VecDeque}, io::{Error, ErrorKind}, str::FromStr, time::Instant};

fn main() {
    part1();
    part2();
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
    let mut i = 0;
    let start = Instant::now();
    while !machine.received_low() {
        i += 1;
        // Reference: Release build: 5M loops in 34s, 10M loops in 69s
        // 1. Update Pulse to store raw pointers to IDs, allowing never cloning values.
        // Result: 5M in 12s, 10M in 23s -- over 2x faster. 50M loops in 121s, 140M loops in 338s.
        // 2. Put pointer to sender in queue, rather than putting 1 entry in queue for each destination.
        // Result: 50M in 83s, 140M in 236s -- ~30% faster again
        if i % 1_000_000 == 0 { println!("Press {}m, t={}s", i/1_000_000, (Instant::now() - start).as_secs()); }
        machine.press();
    }
    println!("Received low output after {i} presses");
}

// Just unrolled directly in Machine::press
// const BUTTON: &str = "button";
const BROADCASTER: &str = "broadcaster";

struct Machine {
    modules: HashMap<String, Box<dyn Module>>,
}

impl Machine {
    fn load() -> Machine {
        let mut modules: HashMap<_,_> = rust_aoc::read_input(20)
            .map(|s| s.parse().unwrap())
            .map(|module: BaseModule| (module.id.clone(), module)).collect();
        // Special output module
        let output_module = BaseModule::new(String::from("rx"), ModuleType::Receiver, vec![]);
        modules.insert(output_module.id.clone(), output_module);

        let map_ptr: *mut _ = &mut modules;
        for id in modules.keys() {
            for downstream in &modules[id].downstream_modules {
                // Safety: we're iterating over keys above, but just want to update the value here.
                // Rust can't tell that these references to keys vs values don't overlap
                let downstream = unsafe { (*map_ptr).get_mut(downstream) };
                downstream.unwrap().add_input(id.clone());
            }
        }

        let modules = modules.into_values().map(|module| (module.id.clone(), module.create())).collect();

        Machine { modules }
    }

    fn press(&mut self) -> (usize, usize) {
        let mut low_count = 1;
        let mut high_count = 0;
        // ISSUE WITH BORROW CHECKER:
        // If queue stores references gotten from self.modules, then there are immutable references to self.modules
        // at all points of the while loop, preventing taking a mutable reference in self.modules.get_mut.
        // Instead, store a raw pointer on the queue given we know the values won't change.
        let mut queue = VecDeque::new();

        let broadcaster = self.modules[BROADCASTER].get_base_module();
        queue.push_back(Pulse { sender: broadcaster, pulse_type: PulseType::Low });
        while let Some(Pulse { sender, pulse_type }) = queue.pop_front() {
            // Keys and graph shape never modified during 'press', only state of individual machines
            let sender = unsafe { &*sender };

            match pulse_type {
                PulseType::High => high_count += sender.downstream_modules.len(),
                PulseType::Low => low_count += sender.downstream_modules.len(),
            }

            for id in &sender.downstream_modules {
                let module = self.modules.get_mut(id).unwrap();
                let output_pulse = module.process(&sender.id, pulse_type);
                if let Some(output_pulse) = output_pulse {
                    queue.push_back(Pulse {
                        sender: module.get_base_module(), 
                        pulse_type: output_pulse
                    });
                }
            }
        }

        (low_count, high_count)
    }

    fn received_low(&self) -> bool {
        self.modules["rx"].received_low()
    }
}

struct BaseModule {
    id: String,
    module_type: ModuleType,
    input_modules: Vec<String>,
    downstream_modules: Vec<String>,
}

impl BaseModule {
    fn new(id: String, module_type: ModuleType, downstream_modules: Vec<String>) -> BaseModule {
        BaseModule { id, module_type, input_modules: vec![], downstream_modules }
    }

    fn add_input(&mut self, id: String) {
        self.input_modules.push(id);
    }

    fn create(self) -> Box<dyn Module> {
        match self.module_type {
            ModuleType::Broadcast => Box::new(BroadcastModule::new(self)),
            ModuleType::Conjunction => Box::new(ConjunctionModule::new(self)),
            ModuleType::FlipFlop => Box::new(FlipFlopModule::new(self)),
            ModuleType::Receiver => Box::new(ReceiverModule::new(self)),
        }
    }
}

impl FromStr for BaseModule {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (id, downstream_modules) = s.split_once(" -> ").unwrap();
        let downstream_modules = downstream_modules.split(", ").map(String::from).collect();
        if id == "broadcaster" {
            Ok(BaseModule::new(String::from(id), ModuleType::Broadcast, downstream_modules))
        } else if id.starts_with('%') {
            Ok(BaseModule::new(String::from(&id[1..]), ModuleType::FlipFlop, downstream_modules))
        } else if id.starts_with('&') {
            Ok(BaseModule::new(String::from(&id[1..]), ModuleType::Conjunction, downstream_modules))
        } else {
            Err(Error::new(ErrorKind::InvalidInput, format!("Unrecognised ID {id}")))
        }
    }
}

trait Module {
    fn new(base: BaseModule) -> Self where Self: Sized;
    fn process(&mut self, from: &str, pulse_type: PulseType) -> Option<PulseType>;
    fn get_base_module(&self) -> &BaseModule;
    fn received_low(&self) -> bool {
        false
    }
}

struct BroadcastModule {
    base: BaseModule,
}

impl Module for BroadcastModule {
    fn new(base: BaseModule) -> Self {
        BroadcastModule { base }
    }

    fn process(&mut self, _from: &str, pulse_type: PulseType) -> Option<PulseType> {
        Some(pulse_type)
    }

    fn get_base_module(&self) -> &BaseModule {
        &self.base
    }
}

struct ConjunctionModule {
    base: BaseModule,
    inputs: HashMap<String, PulseType>,
}

impl Module for ConjunctionModule {
    fn new(base: BaseModule) -> Self {
        let inputs = base.input_modules.iter().map(|id| (id.clone(), PulseType::Low)).collect();
        ConjunctionModule { base, inputs }
    }

    fn process(&mut self, from: &str, pulse_type: PulseType) -> Option<PulseType> {
        *self.inputs.get_mut(from).unwrap() = pulse_type;
        if self.inputs.values().all(|input| *input == PulseType::High) {
            Some(PulseType::Low)
        } else {
            Some(PulseType::High)
        }
    }

    fn get_base_module(&self) -> &BaseModule {
        &self.base
    }
}

struct FlipFlopModule {
    base: BaseModule,
    on: bool,
}

impl Module for FlipFlopModule {
    fn new(base: BaseModule) -> Self {
        FlipFlopModule { base, on: false }
    }

    fn process(&mut self, _from: &str, pulse_type: PulseType) -> Option<PulseType> {
        match pulse_type {
            PulseType::High => None,
            PulseType::Low  => {
                self.on = !self.on;
                Some(if self.on { PulseType::High } else { PulseType::Low })
            }
        }
    }

    fn get_base_module(&self) -> &BaseModule {
        &self.base
    }
}

struct ReceiverModule {
    base: BaseModule,
    received_low: bool,
}

impl Module for ReceiverModule {
    fn new(base: BaseModule) -> Self {
        ReceiverModule { base, received_low: false }
    }

    fn process(&mut self, _from: &str, pulse_type: PulseType) -> Option<PulseType> {
        if pulse_type == PulseType::Low { self.received_low = true }
        None
    }

    fn get_base_module(&self) -> &BaseModule {
        &self.base
    }

    fn received_low(&self) -> bool {
        self.received_low
    }
}

enum ModuleType { Broadcast, Conjunction, FlipFlop, Receiver }

// Safety: IDs of map shouldn't be modified once initially built
struct Pulse {
    sender: *const BaseModule,
    pulse_type: PulseType,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum PulseType { High, Low }
