pub mod instructions;
mod mem;
use instructions::Instruction;
use mem::{Memory, ObjectIndex};
use std::{
    fmt::{Debug, Display},
    thread,
    time::Duration,
    vec,
};

pub struct VM {
    pub stack: Stack,
    pub object_map: Memory,
    constants: Vec<VMData>,
    call_stack: Vec<usize>,
    pc: usize,
}

impl VM {
    pub fn new() -> Self {
        Self {
            stack: Stack::new(),
            object_map: Memory::new(16),
            constants: vec![],
            call_stack: vec![],
            pc: usize::default(),
        }
    }
    #[inline(always)]
    pub fn clean(&mut self) {
        self.stack.top = 1;
        self.call_stack = vec![];
        self.pc = usize::default();
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
            println!("{}", self.stack);

            #[cfg(debug_assertions)]
            thread::sleep(Duration::from_millis(250));
        }
        self.clean();
    }
    pub fn execute_instruction(&mut self, ins: &Instruction) {
        use Instruction::*;
        match ins {
            PushI(i) => self.stack.push(VMData::new_i64(*i)),
            Pop => {
                self.stack.pop().expect("Stack Underflow");
            }
            Print => {
                let value = self.stack.last().expect("Stack Underflow");
                println!("val: {}", value)
            }
            AddI => {
                let b = self.stack.pop().expect("Stack Underflow").as_i64();
                let a = self.stack.pop().expect("Stack Underflow").as_i64();
                self.stack.push(VMData::new_i64(a + b));
            }
            MulI => {
                let b = self.stack.pop().expect("Stack Underflow").as_i64();
                let a = self.stack.pop().expect("Stack Underflow").as_i64();
                self.stack.push(VMData::new_i64(a * b));
            }
            DivI => {
                let b = self.stack.pop().expect("Stack Underflow").as_i64();
                if b == 0 {
                    panic!("Can't divide by 0");
                }
                let a = self.stack.pop().expect("Stack Underflow").as_i64();
                self.stack.push(VMData::new_i64(a / b));
            }
            SubI => {
                let b = self.stack.pop().expect("Stack Underflow").as_i64();
                let a = self.stack.pop().expect("Stack Underflow").as_i64();
                self.stack.push(VMData::new_i64(a - b));
            }
            Dup => {
                let last = self.stack.last().expect("Stack Underflow");
                self.stack.push(*last);
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
            JumpIfTrue(address) => {
                let val = self.stack.pop().expect("Stack Underflow").as_bool();
                if val {
                    self.pc = address.into();
                    return;
                }
            }
            JumpIfFalse(address) => {
                let val = self.stack.pop().expect("Stack Underflow").as_bool();
                if !val {
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
            /*Read => {
                let mut input = String::new();
                std::io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read input");
                let val = input.trim().parse().expect("Invalid input");
                self.stack.push(val);
            }*/
            Instruction::Eq => {
                let b = self.stack.pop().expect("Stack underflow");
                let a = self.stack.pop().expect("Stack underflow");
                self.stack.push(VMData::new_bool(a == b));
            }
            Instruction::Neq => {
                let b = self.stack.pop().expect("Stack underflow");
                let a = self.stack.pop().expect("Stack underflow");
                self.stack.push(VMData::new_bool(a != b));
            }
            Instruction::Lt => {
                let b = self.stack.pop().expect("Stack underflow");
                let a = self.stack.pop().expect("Stack underflow");
                self.stack.push(VMData::new_bool(a < b));
            }
            Instruction::Gt => {
                let b = self.stack.pop().expect("Stack underflow");
                let a = self.stack.pop().expect("Stack underflow");
                self.stack.push(VMData::new_bool(a > b));
            }
            Instruction::Lte => {
                let b = self.stack.pop().expect("Stack underflow");
                let a = self.stack.pop().expect("Stack underflow");
                self.stack.push(VMData::new_bool(a <= b));
            }
            Instruction::Gte => {
                let b = self.stack.pop().expect("Stack underflow");
                let a = self.stack.pop().expect("Stack underflow");
                self.stack.push(VMData::new_bool(a >= b));
            }
            Instruction::And => {
                let b = self.stack.pop().expect("Stack underflow").as_bool();
                let a = self.stack.pop().expect("Stack underflow").as_bool();
                self.stack.push(VMData::new_bool(a && b));
            }
            Instruction::Or => {
                let b = self.stack.pop().expect("Stack underflow").as_bool();
                let a = self.stack.pop().expect("Stack underflow").as_bool();
                self.stack.push(VMData::new_bool(a || b));
            }
            Instruction::Not => {
                let value = self.stack.pop().expect("Stack underflow").as_bool();
                self.stack.push(VMData::new_bool(!value));
            }
            Nop => {}
            _ => unimplemented!(),
        }
        self.pc += 1;
    }
}

const STACK_SIZE: usize = 16 * 1024 / size_of::<VMData>();
#[derive(Debug)]
pub struct Stack {
    values: [VMData; STACK_SIZE],
    pub top: usize,
}

// TODO: this implementation should be overhauled a bit cuz it's kinda clunky
impl Stack {
    pub fn new() -> Self {
        Self {
            values: [VMData::new_unit(); STACK_SIZE],
            top: 1,
        }
    }

    pub fn push(&mut self, val: VMData) {
        if !(self.top >= STACK_SIZE) {
            self.values[self.top] = val;
            self.top += 1;
        } else {
            panic!("Stack full");
        }
    }

    pub fn pop(&mut self) -> Option<VMData> {
        if self.top != 0 {
            self.top -= 1;
            let r = self.values[self.top];
            Some(r)
        } else {
            None
        }
    }

    #[inline(always)]
    pub fn last(&self) -> Option<&VMData> {
        if self.top != 0 {
            Some(&self.values[self.top - 1])
        } else {
            None
        }
    }
}

impl Display for Stack {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Stack: {{ values: {}, top: {}}}",
            {
                let mut s = "[".to_string();
                self.values.into_iter().for_each(|v| {
                    s.push_str(&format!("{}, ", &v.to_string()));
                });
                s.push(']');
                s
            },
            self.top
        )
    }
}

#[derive(Clone, Copy)]
pub union RawVMData {
    as_unit: (),
    as_i64: i64,
    as_u64: u64,
    as_f64: f64,
    as_bool: bool,
    as_object: ObjectIndex,
}

#[derive(Clone, Copy)]
pub struct VMData {
    pub tag: u64,
    data: RawVMData,
}

macro_rules! def_new_vmdata_func {
    ($ident: ident, $field: ident, $ty: ty, $const: ident) => {
        #[inline(always)]
        pub fn $ident(val: $ty) -> Self {
            Self::new(Self::$const, RawVMData { $field: val })
        }
    };
}

impl VMData {
    pub const TAG_UNIT: u64 = 0;
    pub const TAG_U64: u64 = 4;
    pub const TAG_I64: u64 = 8;
    pub const TAG_FLOAT: u64 = 9;
    pub const TAG_BOOL: u64 = 10;
    pub const TAG_STR: u64 = 11;

    pub fn new(tag: u64, data: RawVMData) -> Self {
        Self { tag, data }
    }

    pub fn new_unit() -> Self {
        Self {
            tag: Self::TAG_UNIT,
            data: RawVMData { as_unit: () },
        }
    }

    pub fn new_object(tag: u64, val: ObjectIndex) -> Self {
        assert!(tag > 256, "object typeid is within the reserved area");
        Self {
            tag,
            data: RawVMData { as_object: val },
        }
    }

    pub fn new_string(val: ObjectIndex) -> Self {
        Self::new(Self::TAG_STR, RawVMData { as_object: val })
    }

    def_new_vmdata_func!(new_i64, as_i64, i64, TAG_I64);
    def_new_vmdata_func!(new_u64, as_u64, u64, TAG_U64);
    def_new_vmdata_func!(new_float, as_f64, f64, TAG_FLOAT);
    def_new_vmdata_func!(new_bool, as_bool, bool, TAG_BOOL);
}

impl PartialEq for VMData {
    fn eq(&self, other: &Self) -> bool {
        if self.tag != other.tag {
            return false;
        }

        match self.tag {
            Self::TAG_BOOL => self.as_bool() == other.as_bool(),
            Self::TAG_FLOAT => self.as_f64() == other.as_f64(),
            Self::TAG_I64 => self.as_i64() == other.as_i64(),
            Self::TAG_U64 => self.as_u64() == other.as_u64(),
            Self::TAG_UNIT => true,
            _ if self.tag > 256 => self.as_object() == other.as_object(),
            _ => panic!("Illegal comparison"),
        }
    }
}

impl PartialOrd for VMData {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.tag != other.tag {
            return None;
        }

        match self.tag {
            Self::TAG_FLOAT => self.as_f64().partial_cmp(&other.as_f64()),
            Self::TAG_U64 => self.as_u64().partial_cmp(&other.as_u64()),
            Self::TAG_I64 => self.as_i64().partial_cmp(&other.as_i64()),
            _ => panic!("Illegal comparison"),
        }
    }
}

impl Display for VMData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self.tag {
                Self::TAG_UNIT => "()".to_string(),
                Self::TAG_I64 => self.as_i64().to_string(),
                Self::TAG_U64 => self.as_u64().to_string(),
                Self::TAG_FLOAT => self.as_f64().to_string(),
                Self::TAG_BOOL => self.as_bool().to_string(),

                _ if self.is_object() => self.as_object().to_string(),
                _ => "reserved".to_string(),
            }
        )
    }
}

impl Debug for VMData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "VMData {{ tag: {}({}), data: {}}}",
            self.tag,
            match self.tag {
                Self::TAG_BOOL => "bool",
                Self::TAG_UNIT => "unit",
                Self::TAG_FLOAT => "f64",
                Self::TAG_I64 => "i64",
                Self::TAG_U64 => "u64",
                _ if self.is_object() => "obj",
                _ => "res",
            },
            match self.tag {
                Self::TAG_UNIT => "()".to_string(),
                Self::TAG_I64 => self.as_i64().to_string(),
                Self::TAG_U64 => self.as_u64().to_string(),
                Self::TAG_FLOAT => self.as_f64().to_string(),
                Self::TAG_BOOL => self.as_bool().to_string(),
                _ if self.is_object() => self.as_object().to_string(),
                _ => "reserved".to_string(),
            }
        )
    }
}

macro_rules! enum_variant_function {
    ($getter: ident, $is: ident, $variant: ident, $ty: ty) => {
        #[inline(always)]
        #[must_use]
        pub fn $getter(self) -> $ty {
            unsafe { self.data.$getter }
        }

        #[inline(always)]
        #[must_use]
        pub fn $is(self) -> bool {
            self.tag == Self::$variant
        }
    };
}

impl VMData {
    enum_variant_function!(as_i64, is_i64, TAG_I64, i64);
    enum_variant_function!(as_f64, is_f64, TAG_FLOAT, f64);
    enum_variant_function!(as_u64, is_u64, TAG_U64, u64);
    enum_variant_function!(as_bool, is_bool, TAG_BOOL, bool);
    enum_variant_function!(as_unit, is_unit, TAG_UNIT, ());

    #[inline(always)]
    #[must_use]
    pub fn is_object(self) -> bool {
        self.tag > 256 || self.tag == Self::TAG_STR
    }

    #[inline(always)]
    #[must_use]
    pub fn as_object(self) -> ObjectIndex {
        if !self.is_object() {
            unreachable!()
        }

        unsafe { self.data.as_object }
    }
}
