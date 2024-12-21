use std::io::Error;

use aoc_2024::read_input;

fn main() {
    part1();
    part2();
}

fn part1() {
    let machine = parse_input();
    let output = machine.map(|n| n.to_string()).collect::<Vec<_>>().join(",");
    println!("Ouptut: {:?}", output); // 3,1,4,3,1,7,1,6,3
}

fn part2() {
    let original_machine = parse_input();
    let result = find_fixed_point(original_machine);
    println!("Fixed point: {result}"); // 37221270076916
}

/// From manual inspection of the input:
/// B <- A % 8
/// B <- B xor 2
/// C <- A / 2^B
/// B <- B xor C
/// B <- B xor 3
/// Print B % 8
/// A <- A/8
/// If A != 0, repeat
/// 
/// So prints a value determined by A, then divides A by 8, and repeats until A is 0.
/// Thefore, can find the input giving a certain output by finding the value in 0..8 giving the last digit,
/// and then trying each of (8*n + 0..8) giving the digit before, until we have the whole program.
fn find_fixed_point(original_machine: Machine) -> Int {
    let mut expected_output_rev = original_machine.program.iter().rev();

    let last_instruction = *expected_output_rev.next().unwrap();
    let mut possible_reg: Box<dyn Iterator<Item=Int>> = Box::new((0..8 as Int).filter(|&reg| {
        let mut machine = clone_with_reg_a(&original_machine, reg);
        machine.next() == Some(last_instruction) && machine.next() == None
    }));

    for expected in expected_output_rev {
        possible_reg = Box::new(possible_reg.flat_map(|n| (0..8).map(move |m| 8*n + m))
            .filter(|&reg| {
                let mut machine = clone_with_reg_a(&original_machine, reg);
                machine.next() == Some(*expected)
            }));
    }

    possible_reg.next().unwrap()
}

fn clone_with_reg_a(machine: &Machine, register_a: Int) -> Machine {
    let mut machine = Machine::new(machine.registers.clone(), machine.program.clone());
    machine.registers[0] = register_a;
    machine
}

type Int = u64;

struct Machine {
    registers: [Int; 3],
    program: Vec<Int>,
    pc: usize,
    output: Vec<Int>,
}

impl Machine {
    fn new(registers: [Int; 3], program: Vec<Int>) -> Machine {
        Machine { registers, program, pc: 0, output: Vec::new() }
    }

    fn combo_operand(&self, operand: Int) -> Int {
        match operand {
            0..4 => operand,
            4..7 => self.registers[(operand - 4) as usize],
            _ => panic!("Invalid combo operand {operand}")
        }
    }

    fn step(&mut self) {
        let operator = Operator::try_from(self.program[self.pc]).unwrap();
        let operand = self.program[self.pc + 1];
        let instruction = Instruction { operator, operand };
        instruction.apply(self);
    }

    fn has_next(&self) -> bool {
        self.pc < self.program.len()
    }
}

impl Iterator for Machine {
    type Item = Int;

    fn next(&mut self) -> Option<Self::Item> {
        let next_output_idx = self.output.len();
        while self.has_next() {
            self.step();
            if let Some(&output) = self.output.get(next_output_idx) {
                return Some(output)
            }
        }
        None
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
    Machine::new(registers, program)
}
