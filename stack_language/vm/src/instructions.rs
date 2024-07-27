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