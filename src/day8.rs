use std::fs::read_to_string;

struct Instr {
    op: String,
    val: i32,
    executed: bool,
}

struct Program {
    instructions: Vec<Instr>,
    accumulator: i32,
}

impl Program {
    fn new(instructions: Vec<Instr>) -> Program {
        Program { instructions, accumulator: 0 }
    }

    fn run(&mut self) -> bool {
        let mut pc: i32 = 0;
        let mut instr_to_exec;
        let normal_exit = loop {
            instr_to_exec = &mut self.instructions[pc as usize];
            if instr_to_exec.executed {
                println!("Program terminated before repeating");
                break false;
            }
            // println!("{}:{}:{}", instr_to_exec.op, instr_to_exec.val, instr_to_exec.executed);
            match instr_to_exec.op.as_str() {
                "acc" => {
                    self.accumulator += instr_to_exec.val;
                    pc += 1;
                }
                "jmp" => pc += instr_to_exec.val,
                _ => pc += 1
            }
            instr_to_exec.executed = true;

            if pc >= self.instructions.len() as i32 {
                println!("Program terminated normally");
                break true;
            }
        };
        return normal_exit;
    }
}

pub fn day8(path: &str) {
//     let input = "nop +0
// acc +1
// jmp +4
// acc +3
// jmp -3
// acc -99
// acc +1
// jmp -4
// acc +6";
    let input = read_to_string(path).unwrap();
    let mut last_op_swap_index = 0;
    let mut op_swapped = true;
    loop {
        op_swapped = !op_swapped;
        if op_swapped {
            break;
        }
        let mut instructions: Vec<Instr> = Vec::with_capacity(input.len());
        for (i, line) in input.lines().enumerate() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            let mut op = parts[0].to_string();
            let val = parts[1].parse::<i32>().unwrap();
            if i > last_op_swap_index && !op_swapped {
                match op.as_str() {
                    "nop" => {
                        op = "jmp".to_string();
                        last_op_swap_index = i;
                        op_swapped = true;
                    }
                    "jmp" => {
                        op = "nop".to_string();
                        last_op_swap_index = i;
                        op_swapped = true;
                    }
                    _ => {}
                }
            }
            instructions.push(Instr { op, val, executed: false });
        }

        let mut program = Program::new(instructions);
        if program.run() {
            println!("{}", program.accumulator);
            break;
        }
    }
}