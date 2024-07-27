pub mod instructions;
use core::time;
use std::{thread, time::{ Duration}};

use instructions::Instruction;

pub struct StackMachine {
    pc: usize,
    stack: Vec<i32>,
    call_stack: Vec<usize>
}

impl StackMachine {
    pub fn new() -> Self {
        Self {
            pc: 0,
            stack: Vec::new(),
            call_stack: Vec::new()
        }
    }
    pub fn execute(&mut self, ins: Vec<Instruction>) {
        while self.pc < ins.len() {
            let ins = &ins[self.pc];
            //println!("{:?}", ins);
            match ins {
                Instruction::HLT => break,
                _ => {self.execute_instruction(ins);}
            }
            //println!("{:?}", self.stack);
            //thread::sleep(Duration::from_millis(100));
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
                    self.call_stack.push(self.pc as usize + 1);
                    self.pc = address.into();
                    return;
                }
                Ret => {
                    self.pc = self.call_stack.pop().expect("Call Stack Underflow") as usize;
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
                _ => unimplemented!()
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
    use std::time;
    let instant = time::Instant::now();
    use instructions::{Instruction::*, *};
    let ins = vec![
        Push(40),
        Call(Address::Val(4)),
        Print,
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
    let elapsed = instant.elapsed();
    println!("{:?}", elapsed);
}
