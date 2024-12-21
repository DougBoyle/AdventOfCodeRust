use std::io::Error;

use aoc_2024::read_input;

fn main() {
    part1();
    part2();
}

fn part1() {
    let mut machine = parse_input();
    // let mut machine = OptimisedMachine::new(&machine.program, machine.registers[0]);
    machine.run();
    let output: String = machine.output.iter().map(|n| n.to_string()).collect::<Vec<_>>().join(",");
    println!("Ouptut: {:?}", output); // 3,1,4,3,1,7,1,6,3
}

fn part2() {
    let original_machine = parse_input();
    let result = find_fixed_point(original_machine);
    println!("Fixed point: {result}");
}

fn find_fixed_point(original_machine: Machine) -> Int {
    // validity check
    OptimisedMachine::new(&original_machine.program, &mut Vec::new(), original_machine.registers[0]);

    // optimisation, allocate a single output vector and keep reusing it
    let mut output = Vec::new();

    let print_interval = 10_000_000;
    let min_possible_value = (2 as Int).pow(45); // given that the output has 16 digits
    (min_possible_value..).filter(|start| {
        if start % print_interval == 0 { println!("Trying {}", start/print_interval) };
        try_fixed_point(&original_machine, &mut output, *start)
    }).next().unwrap()
}

fn try_fixed_point(original_machine: &Machine, output: &mut Vec<Int>, start: Int) -> bool {
    output.clear();
    //let mut machine = Machine {
    //    registers: original_machine.registers.clone(),
    //    program: original_machine.program.clone(),
    //    pc: 0,
    //    output: Vec::new(),
    //};
    //machine.registers[0] = start
    let mut machine = OptimisedMachine { register_a: start, output, finished: false };

    if !original_machine.program.iter().all(|&expected| run_until_output(&mut machine, expected)) { return false };
    let final_len = original_machine.program.len();
    while machine.has_next() {
        machine.step();
        if machine.output.len() > final_len { return false; }
    }
    true
}

fn run_until_output(machine: &mut impl Executable, expected: Int) -> bool {
    let original_len = machine.get_output().len();
    while machine.has_next() {
        machine.step();
        if machine.get_output().len() > original_len {
            return machine.get_output()[original_len] == expected
        }
    }
    return false;
}

type Int = u64;

trait Executable {
    fn step(&mut self);
    fn has_next(&self) -> bool;
    fn get_output(&self) -> &Vec<Int>;

    fn run(&mut self) {
        while self.has_next() {
            self.step();
        }
    }
}

struct Machine {
    registers: [Int; 3],
    program: Vec<Int>,
    pc: usize,
    output: Vec<Int>,
}

impl Machine {
    fn combo_operand(&self, operand: Int) -> Int {
        match operand {
            0..4 => operand,
            4..7 => self.registers[(operand - 4) as usize],
            _ => panic!("Invalid combo operand {operand}")
        }
    }
}

impl Executable for Machine {
    fn step(&mut self) {
        let operator = Operator::try_from(self.program[self.pc]).unwrap();
        let operand = self.program[self.pc + 1];
        let instruction = Instruction { operator, operand };
        instruction.apply(self);
    }

    fn has_next(&self) -> bool {
        self.pc < self.program.len()
    }

    fn get_output(&self) -> &Vec<Int> {
        &self.output
    }
}

struct OptimisedMachine<'a> {
    register_a: Int,
    output: &'a mut Vec<Int>,
    finished: bool,
}

impl<'a> OptimisedMachine<'a> {
    fn expected_program() -> Vec<Int> {
        vec![2,4,1,2,7,5,4,5,1,3,5,5,0,3,3,0]
    }

    fn new(program: &Vec<Int>, output: &'a mut Vec<Int>, register_a: Int) -> OptimisedMachine<'a> {
        assert_eq!(&OptimisedMachine::expected_program(), program, "Optimised machine only works for a specific input program");
        OptimisedMachine { register_a, output, finished: false }
    }
}

impl Executable for OptimisedMachine<'_> {
    fn step(&mut self) {
        let b = (self.register_a % 8) ^ 2;
        let c = self.register_a / (2 as Int).pow(b as u32);
        let b = b ^ c ^ 3;
        self.output.push(b % 8);
        self.register_a /= 8;
        self.finished = self.register_a == 0;
    }

    fn has_next(&self) -> bool {
        !self.finished
    }

    fn get_output(&self) -> &Vec<Int> {
        &self.output
    }
}

#[derive(Copy, Clone, Debug)]
struct Instruction {
    operator: Operator,
    operand: Int,
}

impl Instruction {
    fn apply(&self, machine: &mut Machine) {
        let Instruction { operator, operand } = *self;
        match operator {
            Operator::Adv => machine.registers[0] = self.div_reg_a(machine),
            Operator::Bxl => machine.registers[1] ^= operand,
            Operator::Bst => machine.registers[1] = machine.combo_operand(operand) % 8,
            Operator::Jnz => {
                if machine.registers[0] != 0 {
                    machine.pc = operand as usize;
                    return;
                }
            },
            Operator::Bxc => machine.registers[1] ^= machine.registers[2],
            Operator::Out => machine.output.push(machine.combo_operand(operand) % 8),
            Operator::Bdv => machine.registers[1] = self.div_reg_a(machine),
            Operator::Cdv => machine.registers[2] = self.div_reg_a(machine),
        }
        machine.pc += 2;
    }

    fn div_reg_a(&self, machine: &mut Machine) -> Int {
        machine.registers[0] / (2 as Int).pow(machine.combo_operand(self.operand) as u32)
    }
}

#[derive(Copy, Clone, Debug)]
enum Operator {
    Adv, Bxl, Bst, Jnz, Bxc, Out, Bdv, Cdv,
}

impl TryFrom<Int> for Operator {
    type Error = Error;

    fn try_from(value: Int) -> Result<Self, Self::Error> {
        Ok(match value {
            0 => Operator::Adv,
            1 => Operator::Bxl,
            2 => Operator::Bst,
            3 => Operator::Jnz,
            4 => Operator::Bxc,
            5 => Operator::Out,
            6 => Operator::Bdv,
            7 => Operator::Cdv,
            _ => Err(Error::new(std::io::ErrorKind::InvalidInput, format!("Unrecognised operator {value}")))?
        })
    }
}

fn parse_input() -> Machine {
    let mut lines = read_input(17);
    let registers = lines.by_ref().take(3)
        .map(|line| line.split_ascii_whitespace().last().unwrap().parse().unwrap())
        .collect::<Vec<_>>().try_into().unwrap();
    lines.next(); // skip blank line
    let program = lines.next().unwrap().split_ascii_whitespace().last().unwrap()
        .split(",")
        .map(|n| n.parse::<Int>().unwrap())
        .collect();
    Machine { registers, program, pc: 0, output: Vec::new() }
}
