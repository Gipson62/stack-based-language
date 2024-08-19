#[derive(Debug)]
pub enum Instruction {
    PushI(i64),
    PushU(u64),
    PushF(f64),
    Pop,
    AddI,
    AddU,
    AddF,
    SubI,
    SubU,
    SubF,
    MulI,
    MulU,
    MulF,
    DivI,
    DivU,
    DivF,
    //Duplicate the top value of the stack
    Dup,
    //Swap the 2 top value of the stack (using the tmp register)
    Swap,
    //Rotate the first 3 elements, so [1, 2, 3] => [3, 2, 1]
    Rot,
    Jmp(Address),
    JumpIfTrue(Address),
    JumpIfFalse(Address),
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
    HLT,
    Nop,
}

#[derive(Debug, Default)]
pub enum Address {
    #[default]
    ToDefine,
    Val(usize),
}

impl From<&Address> for usize {
    fn from(value: &Address) -> Self {
        match value {
            Address::ToDefine => panic!(),
            Address::Val(addr) => *addr,
        }
    }
}
