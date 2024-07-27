use std::default;

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
    Jmp(Address),
    JmpIfZero(Address),
    JmpIfNotZero(Address),
    Call(Address),
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

#[derive(Debug, Default)]
pub enum Address {
    #[default]
    ToDefine,
    Val(usize)
}

impl Into<usize> for &Address {
    fn into(self) -> usize {
        match self {
            Address::ToDefine => panic!(),
            Address::Val(addr) => *addr
        }
    }
}