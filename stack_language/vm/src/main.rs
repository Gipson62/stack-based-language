pub mod instructions;
use instructions::Instruction;

pub struct StackMachine {
    pc: usize,
    ins: Vec<Instruction>,
    stack: Vec<i32>,
}

impl StackMachine {
    pub fn new() -> Self {
        Self {
            pc: 0,
            ins: Vec::new(),
            stack: Vec::new(),
        }
    }
    pub fn load_instructions(&mut self, ins: Vec<Instruction>) {
        self.ins = ins;
    }
    
    pub fn execute(&mut self) {
        while (self.pc) < self.ins.len() {
            let ins = &self.ins[self.pc];
            use Instruction::*;
            match ins {
                Push(i) => self.stack.push(*i),
                Pop => {
                    self.stack.pop().expect("Stack Underflow");
                },
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
                },
                Dup => {
                    self.stack.push(*self.stack.last().expect("Stack Underflow"));
                },
                Swap => {
                    let a = self.stack.pop().expect("Stack Underflow");
                    let b = self.stack.pop().expect("Stack Underflow");
                    self.stack.push(a);
                    self.stack.push(b);
                },
                Jmp(address) => {

                    self.pc = address.into();
                    continue;
                },
                JmpIfZero(address) => {
                    let val = self.stack.pop().expect("Stack Underflow");
                    if val == 0 {
                        self.pc = address.into();
                    }
                    continue;
                },
                JmpIfNotZero(address) => {
                    let val = self.stack.pop().expect("Stack Underflow");
                    if val != 0 {
                        self.pc = address.into();
                    }
                    continue;
                },
                Call(address) => {
                    self.stack.push(self.pc as i32 + 1);
                    self.pc = address.into();
                    continue;
                },
                Ret => {
                    self.pc = self.stack.pop().expect("Stack Underflow") as usize;
                    continue;
                },
                Read => {
                    let mut input = String::new();
                    std::io::stdin().read_line(&mut input).expect("Failed to read input");
                    let val = input.trim().parse().expect("Invalid input");
                    self.stack.push(val);
                },
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
                //_ => unimplemented!()
            }
            self.pc += 1;
        }
    }
}

impl Default for StackMachine {
    fn default() -> Self {
        Self::new()
    }
}

fn main() {
    let ins = vec![
        Instruction::Push(2),
        Instruction::Push(3),
        Instruction::Add,
        Instruction::Push(4),
        Instruction::Mul,
        Instruction::Print,
    ];
    let mut stack_machine = StackMachine::new();
    stack_machine.load_instructions(ins);
    stack_machine.execute();
}
