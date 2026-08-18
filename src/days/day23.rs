use std::fs::read_to_string;

struct Instruction;

impl Instruction {
    const HLF: u8 = 0;
    const TPL: u8 = 1;
    const INC: u8 = 2;
    const JMP: u8 = 3;
    const JIE: u8 = 4;
    const JIO: u8 = 5;
}

#[derive(Debug)]
struct Cpu {
    reg_a: u32,
    reg_b: u32,
    pc: usize,
}

impl Cpu {
    fn new() -> Self {
        Cpu {
            reg_a: 0,
            reg_b: 0,
            pc: 0,
        }
    }

    fn pc_index(&self) -> usize {
        self.pc * 3
    }

    fn execute(&mut self, program: &[u8]) -> bool {
        if self.pc_index() > program.len() - 3 {
            return false;
        }

        match program[self.pc_index()] {
            Instruction::HLF => {
                let register = self.get_register(&program[self.pc_index() + 1]);

                *register /= 2;
                self.pc += 1;
                true
            }
            Instruction::TPL => {
                let register = self.get_register(&program[self.pc_index() + 1]);

                *register *= 3;
                self.pc += 1;
                true
            }
            Instruction::INC => {
                let register = self.get_register(&program[self.pc_index() + 1]);

                *register += 1;
                self.pc += 1;
                true
            }
            Instruction::JMP => {
                self.jump(&program[self.pc_index() + 2]);
                true
            }
            Instruction::JIE => {
                let register = self.get_register(&program[self.pc_index() + 1]);

                if !(*register).is_multiple_of(2) {
                    self.pc += 1;
                    return true;
                }

                self.jump(&program[self.pc_index() + 2]);
                true
            }
            Instruction::JIO => {
                let register = self.get_register(&program[self.pc_index() + 1]);

                if *register != 1 {
                    self.pc += 1;
                    return true;
                }

                self.jump(&program[self.pc_index() + 2]);
                true
            }
            _ => false,
        }
    }

    fn get_register(&mut self, register_id: &u8) -> &mut u32 {
        match register_id {
            0 => &mut self.reg_a,
            1 => &mut self.reg_b,
            _ => panic!("Unknown register {register_id}"),
        }
    }

    fn jump(&mut self, offset: &u8) {
        let jump = *offset as i8 as isize;

        self.pc = self.pc.checked_add_signed(jump).unwrap();
    }
}

fn compile_program() -> Vec<u8> {
    let program_data = read_to_string("data/day23.txt").unwrap();

    let mut program = vec![0; program_data.lines().count() * 3];

    let mut pc = 0;

    for line in program_data.lines() {
        if line.is_empty() {
            continue;
        }

        let mut line_parts = line.split(" ");

        match line_parts.next().unwrap() {
            "hlf" => {
                program[pc] = Instruction::HLF;
                program[pc + 1] = get_register(line_parts.next().unwrap());
            }
            "tpl" => {
                program[pc] = Instruction::TPL;
                program[pc + 1] = get_register(line_parts.next().unwrap());
            }
            "inc" => {
                program[pc] = Instruction::INC;
                program[pc + 1] = get_register(line_parts.next().unwrap());
            }
            "jmp" => {
                program[pc] = Instruction::JMP;
                program[pc + 2] = line_parts.next().unwrap().parse::<i8>().unwrap() as u8;
            }
            "jie" => {
                program[pc] = Instruction::JIE;
                program[pc + 1] = get_register(line_parts.next().unwrap().trim_matches(','));
                program[pc + 2] = line_parts.next().unwrap().parse::<i8>().unwrap() as u8;
            }
            "jio" => {
                program[pc] = Instruction::JIO;
                program[pc + 1] = get_register(line_parts.next().unwrap().trim_matches(','));
                program[pc + 2] = line_parts.next().unwrap().parse::<i8>().unwrap() as u8;
            }
            _ => {}
        }

        pc += 3;
    }

    program
}

fn get_register(register_name: &str) -> u8 {
    match register_name {
        "a" => 0,
        "b" => 1,
        _ => panic!("Unknown register {register_name}"),
    }
}

pub fn part1() {
    let program = compile_program();

    let mut cpu = Cpu::new();

    while cpu.execute(&program) {
        println!("{:?}", cpu);
    }
}

pub fn part2() {
    let program = compile_program();

    let mut cpu = Cpu {
        reg_a: 1,
        ..Cpu::new()
    };

    while cpu.execute(&program) {
        println!("{:?}", cpu);
    }
}
