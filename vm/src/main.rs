pub mod instructions;
pub mod lexer;
use lexer::{identifier_system, AtlasLexer, Token, TokenKind};
use std::{env, fs, thread, time::Duration};
pub mod memory;
pub mod parser;

use instructions::Instruction;

#[derive(Debug)]
pub struct StackMachine {
    pc: usize,
    stack: Vec<i32>,
    call_stack: Vec<usize>,
    memory: Vec<i32>,
}

impl StackMachine {
    pub fn new() -> Self {
        Self {
            pc: 0,
            stack: Vec::new(),
            call_stack: Vec::new(),
            memory: Vec::new(),
        }
    }

    pub fn execute(&mut self, ins: Vec<Instruction>) {
        while self.pc < ins.len() {
            let ins = &ins[self.pc];
            #[cfg(debug_assertions)]
            println!("{:?}", ins);
            match ins {
                Instruction::HLT => break,
                _ => {
                    self.execute_instruction(ins);
                }
            }
            #[cfg(debug_assertions)]
            println!("{:?}", self.stack);

            #[cfg(debug_assertions)]
            thread::sleep(Duration::from_millis(25));
        }
    }
    pub fn execute_instruction(&mut self, ins: &Instruction) {
        use Instruction::*;
        match ins {
            Push(i) => self.stack.push(*i),
            Pop => {
                self.stack.pop().expect("Stack Underflow");
            }
            Print => {
                let value = self.stack.last().expect("Stack Underflow");
                println!("{}", value)
            }
            Add => {
                let b = self.stack.pop().expect("Stack Underflow");
                let a = self.stack.pop().expect("Stack Underflow");
                self.stack.push(a + b);
            }
            Mul => {
                let b = self.stack.pop().expect("Stack Underflow");
                let a = self.stack.pop().expect("Stack Underflow");
                self.stack.push(a * b);
            }
            Div => {
                let b = self.stack.pop().expect("Stack Underflow");
                let a = self.stack.pop().expect("Stack Underflow");
                self.stack.push(a / b);
            }
            Sub => {
                let b = self.stack.pop().expect("Stack Underflow");
                let a = self.stack.pop().expect("Stack Underflow");
                self.stack.push(a - b);
            }
            Dup => {
                self.stack
                    .push(*self.stack.last().expect("Stack Underflow"));
            }
            Swap => {
                let a = self.stack.pop().expect("Stack Underflow");
                let b = self.stack.pop().expect("Stack Underflow");
                self.stack.push(a);
                self.stack.push(b);
            }
            Rot => {
                let a = self.stack.pop().expect("Stack Underflow");
                let b = self.stack.pop().expect("Stack Underflow");
                let c = self.stack.pop().expect("Stack Underflow");
                self.stack.push(c);
                self.stack.push(b);
                self.stack.push(a);
            }
            Jmp(address) => {
                self.pc = address.into();
                return;
            }
            JmpIfZero(address) => {
                let val = self.stack.pop().expect("Stack Underflow");
                if val == 0 {
                    self.pc = address.into();
                    return;
                }
            }
            JmpIfNotZero(address) => {
                let val = self.stack.pop().expect("Stack Underflow");
                if val != 0 {
                    self.pc = address.into();
                    return;
                }
            }
            Call(address) => {
                self.call_stack.push(self.pc + 1);
                self.pc = address.into();
                return;
            }
            Ret => {
                self.pc = self.call_stack.pop().expect("Call Stack Underflow");
                return;
            }
            Read => {
                let mut input = String::new();
                std::io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read input");
                let val = input.trim().parse().expect("Invalid input");
                self.stack.push(val);
            }
            Instruction::Eq => {
                let b = self.stack.pop().expect("Stack underflow");
                let a = self.stack.pop().expect("Stack underflow");
                self.stack.push((a == b) as i32);
            }
            Instruction::Neq => {
                let b = self.stack.pop().expect("Stack underflow");
                let a = self.stack.pop().expect("Stack underflow");
                self.stack.push((a != b) as i32);
            }
            Instruction::Lt => {
                let b = self.stack.pop().expect("Stack underflow");
                let a = self.stack.pop().expect("Stack underflow");
                self.stack.push((a < b) as i32);
            }
            Instruction::Gt => {
                let b = self.stack.pop().expect("Stack underflow");
                let a = self.stack.pop().expect("Stack underflow");
                self.stack.push((a > b) as i32);
            }
            Instruction::Lte => {
                let b = self.stack.pop().expect("Stack underflow");
                let a = self.stack.pop().expect("Stack underflow");
                self.stack.push((a <= b) as i32);
            }
            Instruction::Gte => {
                let b = self.stack.pop().expect("Stack underflow");
                let a = self.stack.pop().expect("Stack underflow");
                self.stack.push((a >= b) as i32);
            }
            Instruction::And => {
                let b = self.stack.pop().expect("Stack underflow");
                let a = self.stack.pop().expect("Stack underflow");
                self.stack.push((a != 0 && b != 0) as i32);
            }
            Instruction::Or => {
                let b = self.stack.pop().expect("Stack underflow");
                let a = self.stack.pop().expect("Stack underflow");
                self.stack.push((a != 0 || b != 0) as i32);
            }
            Instruction::Not => {
                let value = self.stack.pop().expect("Stack underflow");
                self.stack.push((value == 0) as i32);
            }
            Nop => {}
            _ => unimplemented!(),
        }
        self.pc += 1;
    }
}

impl Default for StackMachine {
    fn default() -> Self {
        Self::new()
    }
}

fn main() {
    //env::set_var("RUST_BACKTRACE", "1");
    /*let mut tmp = Duration::new(0, 0);
    for _ in 0..10 {
        use std::time;
        let instant = time::Instant::now();
        use instructions::{Instruction::*, *};
        let ins = vec![
            Push(40),
            Call(Address::Val(4)),
            Nop,
            HLT,
            Dup,
            Push(2),
            Lt,
            JmpIfZero(Address::Val(9)),
            Ret,
            Dup,
            Push(1),
            Sub,
            Call(Address::Val(4)),
            Swap,
            Push(2),
            Sub,
            Call(Address::Val(4)),
            Add,
            Ret,
        ];
        let mut stack_machine = StackMachine::new();
        stack_machine.execute(ins);
        tmp += instant.elapsed();
    }
    println!("{:?}", tmp.div_f32(100.0));*/

    if let Ok(content) = fs::read_to_string("./vm/src/example.txt") {
        let mut lexer = AtlasLexer::default();
        lexer.set_path("src/example.txt");
        lexer.set_source(content);
        lexer.add_system(identifier_system);
        let res = lexer.tokenize();
        match res {
            Ok(t) => t.into_iter().for_each(|tok| println!("{:?}", tok.kind())),
            Err(_e) => {
                println!("Error1");
            }
        }
    } else {
        println!("Error2")
    }
}
