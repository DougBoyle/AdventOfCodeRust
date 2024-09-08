use std::{borrow::Borrow, cell::RefCell, collections::{HashMap, VecDeque}, io::{Error, ErrorKind}, rc::Rc, str::FromStr, time::Instant};

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
        // Result: 50M in 83s, 140M in 236s -- ~30% faster again. 200M in 338s, still no result after 830M in 1230s.
        // 3. Use Rc to point to other nodes directly, rather than going via HashMap, 
        //    and RefCell to do updates on 'immutable' instances rather than needing unsafe raw mut pointers.
        // Result: 50M in 31s, 200M in 125s -- ~2.5x faster
        if i % 1_000_000 == 0 { println!("Press {}m, t={}s", i/1_000_000, (Instant::now() - start).as_secs()); }
        machine.press();
    }
    println!("Received low output after {i} presses");
}

// Just unrolled directly in Machine::press
// const BUTTON: &str = "button";
const BROADCASTER: &str = "broadcaster";

struct Machine {
    modules: HashMap<String, Rc<dyn Module>>,
}

impl Machine {
    fn load() -> Machine {
        let mut parsed_modules: HashMap<_,_> = rust_aoc::read_input(20)
            .map(|s| s.parse().unwrap())
            .map(|module: ParsedModule| (module.id.clone(), module)).collect();
        // Special output module
        let output_module = ParsedModule::new(String::from("rx"), ModuleType::Receiver, vec![]);
        parsed_modules.insert(output_module.id.clone(), output_module);

        let map_ptr: *mut _ = &mut parsed_modules;
        for id in parsed_modules.keys() {
            for downstream in &parsed_modules[id].downstream_modules {
                // Safety: we're iterating over keys above, but just want to update the value here.
                // Rust can't tell that these references to keys vs values don't overlap
                let downstream = unsafe { (*map_ptr).get_mut(downstream) };
                downstream.unwrap().add_input(id.clone());
            }
        }

        // Safety is same as above
        let mut modules: HashMap<_,_> = parsed_modules.values().map(|parsed_module| (parsed_module.id.clone(), parsed_module.create_module())).collect();
        // Pointers set up before any links joined together and Rc's stop being sharable
        let module_ptrs: HashMap<String, *mut dyn Module> = modules.values_mut()
            .map(|module| (module.get_base_module().id.clone(), Rc::get_mut(module).unwrap() as *mut _))
            .collect();
        for parsed_module in parsed_modules.values() {
            let module_ptr: *mut _ = module_ptrs[&parsed_module.id];
         //   let mut module = unsafe { (*map_ptr).get_mut(&parsed_module.id).unwrap() };
            for input in &parsed_module.input_modules {
                unsafe { (*module_ptr).get_base_module_mut().add_input(&modules[input]); }
            }
            for output in &parsed_module.downstream_modules {
                unsafe { (*module_ptr).get_base_module_mut().add_output(&modules[output]); }
            }
        }

        Machine { modules }
    }

    fn press(&mut self) -> (usize, usize) {
        let mut low_count = 1;
        let mut high_count = 0;
        let mut queue = VecDeque::new();

        let broadcaster = &(*self.modules[BROADCASTER]);
        queue.push_back(Pulse { sender: broadcaster, pulse_type: PulseType::Low });
        while let Some(Pulse { sender, pulse_type }) = queue.pop_front() {
            let sender_base = sender.get_base_module();

            match pulse_type {
                PulseType::High => high_count += sender_base.downstream_modules.len(),
                PulseType::Low => low_count += sender_base.downstream_modules.len(),
            }

            for module in &sender_base.downstream_modules {
                let output_pulse = module.process(&sender_base.id, pulse_type);
                if let Some(output_pulse) = output_pulse {
                    queue.push_back(Pulse {
                        sender: module.borrow(), 
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

    fn create_module(&self) -> Rc<dyn Module> {
        let base_module = BaseModule::new(self.id.clone(), self.module_type);
        match base_module.module_type {
            ModuleType::Broadcast => Rc::new(BroadcastModule::new(base_module, self)),
            ModuleType::Conjunction => Rc::new(ConjunctionModule::new(base_module, self)),
            ModuleType::FlipFlop => Rc::new(FlipFlopModule::new(base_module, self)),
            ModuleType::Receiver => Rc::new(ReceiverModule::new(base_module, self)),
        }
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

struct BaseModule {
    id: String,
    module_type: ModuleType,
    input_modules: Vec<Rc<dyn Module>>,
    downstream_modules: Vec<Rc<dyn Module>>, // TODO: Creates a cycle, don't worry about it for now
}

impl BaseModule {
    fn new(id: String, module_type: ModuleType) -> BaseModule {
        BaseModule { id, module_type, input_modules: vec![], downstream_modules: vec![] }
    }

    fn add_input(&mut self, input: &Rc<dyn Module>) {
        self.input_modules.push(Rc::clone(input));
    }

    fn add_output(&mut self, output: &Rc<dyn Module>) {
        self.downstream_modules.push(Rc::clone(output));
    }

  //  fn create(self) -> Rc<RefCell<dyn Module>> {
  //      match self.module_type {
  //          ModuleType::Broadcast => Rc::new(RefCell::new(BroadcastModule::new(self))),
  //          ModuleType::Conjunction => Rc::new(RefCell::new(ConjunctionModule::new(self))),
  //          ModuleType::FlipFlop => Rc::new(RefCell::new(FlipFlopModule::new(self))),
 //           ModuleType::Receiver => Rc::new(RefCell::new(ReceiverModule::new(self))),
 //       }
 //   }
}

/* 
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
    */

trait Module {
    fn new(base: BaseModule, parsed: &ParsedModule) -> Self where Self: Sized;
    fn process(&self, from: &str, pulse_type: PulseType) -> Option<PulseType>;
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
    fn new(base: BaseModule, parsed: &ParsedModule) -> Self {
        BroadcastModule { base }
    }

    fn process(&self, _from: &str, pulse_type: PulseType) -> Option<PulseType> {
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
    inputs: RefCell<HashMap<String, PulseType>>,
}

impl Module for ConjunctionModule {
    fn new(base: BaseModule, parsed: &ParsedModule) -> Self {
        let inputs = parsed.input_modules.iter().map(|module| (module.clone(), PulseType::Low)).collect();
        ConjunctionModule { base, inputs: RefCell::new(inputs) }
    }

    fn process(&self, from: &str, pulse_type: PulseType) -> Option<PulseType> {
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
    on: RefCell<bool>,
}

impl Module for FlipFlopModule {
    fn new(base: BaseModule, parsed: &ParsedModule) -> Self {
        FlipFlopModule { base, on: RefCell::new(false) }
    }

    fn process(&self, _from: &str, pulse_type: PulseType) -> Option<PulseType> {
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
    fn new(base: BaseModule, parsed: &ParsedModule) -> Self {
        ReceiverModule { base, received_low: RefCell::new(false) }
    }

    fn process(&self, _from: &str, pulse_type: PulseType) -> Option<PulseType> {
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

// Safety: IDs of map shouldn't be modified once initially built
struct Pulse<'a> {
    sender: &'a dyn Module,
    pulse_type: PulseType,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum PulseType { High, Low }
