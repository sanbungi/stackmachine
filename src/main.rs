fn get_opcode(mnemonic: &str) -> u8 {
    match mnemonic {
        "HALT" => 0x00,
        "PUSH" => 0x01,
        "POP" => 0x02,
        "ADD" => 0x03,
        "SUB" => 0x04,
        "MUL" => 0x05,
        "DIV" => 0x06,
        "JMP" => 0x07,
        "JZ" => 0x08,
        "PRINT" => 0x09,
        "LOAD" => 0x0A,
        "STORE" => 0x0B,
        _ => panic!("invalid opcode: {}", mnemonic),
    }
}

fn assemble_line(mnemonic: &str, data: u32) -> u32 {
    let opcode = get_opcode(mnemonic);
    let res = ((opcode as u32) << 24) | (data & 0xFFFFFF);

    // Debug
    let bytes = res.to_be_bytes();
    println!("--- Binary Layout ---");
    println!(
        "Hex:  {:02X} {:02X} {:02X} {:02X} → [{} {}]",
        bytes[0], bytes[1], bytes[2], bytes[3],mnemonic,data
    );
    println!("Bin:  {:08b} | {:024b} | {}", opcode, data, data);

    assert_eq!(bytes.len(), 4);
    return res;
}

struct VM {
    pc: usize,
    stack: Vec<u32>,
    halted: bool,
    memory: Vec<u32>,
}

impl VM {
    fn run(&mut self) {
        while !self.halted && self.pc <= self.memory.len() {
            let instruction = self.fetch();
            self.execute(instruction)
        }
    }
    fn fetch(&mut self) -> u32 {
        let inst = self.memory[self.pc];
        self.pc += 1;
        inst
    }
    fn execute(&mut self, instr: u32) {
        let opcode = (instr >> 24) as u8;
        let data = instr & 0xFFFFFF;

        match opcode {
            0x00 => {
                self.halted = true;
                println!("HALT!");
            }
            0x01 => {
                self.stack.push(data);
                println!("PUSH R0, {:06X}", data);
            }
            0x03 => {
                let b = self.stack.pop().unwrap_or(0);
                let a = self.stack.pop().unwrap_or(0);
                self.stack.push(a + b);
                println!("ADD STACK, {:06X} + {:06X}  (Result: {})", a, b, a + b);
            }
            0x04 => {
                let b = self.stack.pop().unwrap_or(0);
                let a = self.stack.pop().unwrap_or(0);
                self.stack.push(a - b);
                println!("SUB STACK, {:06X} - {:06X}  (Result: {})", a, b, a - b);
            }
            0x05 => {
                let b = self.stack.pop().unwrap_or(0);
                let a = self.stack.pop().unwrap_or(0);
                self.stack.push(a * b);
                println!("MUL STACK, {:06X} * {:06X}  (Result: {})", a, b, a * b);
            }
            0x06 => {
                let b = self.stack.pop().unwrap_or(0);
                let a = self.stack.pop().unwrap_or(0);
                self.stack.push(a / b);
                println!("DIV STACK, {:06X} / {:06X}  (Result: {})", a, b, a / b);
            }
            0x07 => {
                let target = data as usize;
                println!("JMP {:06X} to {:06X}", self.pc, target);
                self.pc = target;
            }
            0x08 => {
                let target = data as usize;
                if let Some(val) = self.stack.pop() {
                    if val == 0 {
                        self.pc = target;
                    }
                }
            }
            0x09 => {
                println! {"STACK IS, {:?}",self.stack}
            }
            0x0A => {
                let addr = data as usize;
                if addr < self.memory.len() {
                    let val = self.memory[addr];
                    self.stack.push(val);
                }
            }
            0x0B => {
                let addr = data as usize;
                if addr < self.memory.len() {
                    if let Some(val) = self.stack.pop() {
                        self.memory[addr] = val;
                    }
                }
            }
            _ => println!("Unknown Opcode: {:02X}", opcode),
        }
    }
}

use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    // 引数チェック
    if args.len() < 2 {
        eprintln!("Usage: {} <FilePath>", args[0]);
        return;
    }

    let input = match fs::read_to_string(&args[1]) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error: {}", e);
            return;
        }
    };

    let mut memory = vec![0u32; 1024];
    let mut i = 0;

    for line in input.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }

        let mnemonic = parts[0];
        let data: u32 = parts.get(1).unwrap_or(&"0").parse().unwrap_or(0);

        memory[i] = assemble_line(mnemonic, data);
        i += 1;
    }

    let mut vm = VM {
        pc: 0,
        stack: Vec::with_capacity(256),
        halted: false,
        memory,
    };
    vm.run();
}
