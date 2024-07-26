#[derive(Debug)]
pub enum Instruction {
    Push(i32),
    Pop,
    Add,
    Sub,
    Mul,
    Div,
    //Duplicate the top value of the stack
    Dup,
    //Swap the 2 top value of the stack (using the tmp register)
    Swap,
    //Copies the second top value of the stack and push it on top of it
    Over,
    Jmp(usize),
    JmpIfZero(usize),
    JmpIfNotZero(usize),
    Call(usize),
    Ret,
    Print,
    Read,
    Eq,
    Neq,
    Lt,
    Gt,
    Lte,
    Gte,
    And,
    Or,
    Not,
}

pub struct StackMachine {
    pc: i32,
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
        while (self.pc as usize) < self.ins.len() {
            let ins = &self.ins[self.pc as usize];
            match ins {
                Instruction::Push(i) => self.stack.push(*i),
                Instruction::Print => {
                    let value = self.stack.pop().expect("Stack Underflow");
                    println!("{}", value)
                }
                Instruction::Add => {
                    let b = self.stack.pop().expect("Stack Underflow");
                    let a = self.stack.pop().expect("Stack Underflow");
                    self.stack.push(a + b);
                }
                Instruction::Mul => {
                    let b = self.stack.pop().expect("Stack Underflow");
                    let a = self.stack.pop().expect("Stack Underflow");
                    self.stack.push(a * b);
                }
                Instruction::Div => {
                    let b = self.stack.pop().expect("Stack Underflow");
                    let a = self.stack.pop().expect("Stack Underflow");
                    self.stack.push(a / b);
                }
                Instruction::Sub => {
                    let b = self.stack.pop().expect("Stack Underflow");
                    let a = self.stack.pop().expect("Stack Underflow");
                    self.stack.push(a - b);
                },
                _ => unimplemented!()
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
