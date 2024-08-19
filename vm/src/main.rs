use std::time::Duration;

use vm::instruction::{Address, Instruction::*};


use vm::runtime::VM;

const TEST_AMOUNT: usize = 10;

fn main() {
    //env::set_var("RUST_BACKTRACE", "1");
    let mut tmp = Duration::new(0, 0);
    let mut stack_machine = VM::new();
    let ins = vec![
        PushI(40),
        Call(Address::Val(4)),
        Nop,
        HLT,
        Dup,
        PushI(2),
        Lt,
        JumpIfFalse(Address::Val(9)),
        Ret,
        Dup,
        PushI(1),
        SubI,
        Call(Address::Val(4)),
        Swap,
        PushI(2),
        SubI,
        Call(Address::Val(4)),
        AddI,
        Ret,
    ];
    for _ in 0..TEST_AMOUNT {
        use std::time;
        let ins = ins.clone();
        let instant = time::Instant::now();
        stack_machine.execute(ins);
        tmp += instant.elapsed();
        stack_machine.clean();
    }
    println!("tmp1: {:?}", tmp.div_f32(TEST_AMOUNT as f32));

    let mut tmp = Duration::new(0, 0);
    for _ in 0..TEST_AMOUNT {
        let instant = std::time::Instant::now();
        fibonacci(40);
        tmp += instant.elapsed();
    }
    println!("tmp2: {:?}", tmp.div_f32(TEST_AMOUNT as f32));

    /*if let Ok(content) = std::fs::read_to_string("./vm/src/example.txt") {
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
    }*/
}

fn fibonacci(n: i64) -> i64 {
    if n < 2 {
        return n;
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}
