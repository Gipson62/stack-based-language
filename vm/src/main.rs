pub mod lexer;
use std::time::Duration;

use vm::instructions::{Address, Instruction::*};

use lexer::{identifier_system, AtlasLexer};
pub mod parser;

use vm::VM;

fn main() {
    //env::set_var("RUST_BACKTRACE", "1");
    let mut tmp = Duration::new(0, 0);
    for _ in 0..100 {
        use std::time;
        let instant = time::Instant::now();
        let ins = vec![
            PushI(20),
            Call(Address::Val(4)),
            Print,
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
        let mut stack_machine = VM::new();
        stack_machine.execute(ins);
        tmp += instant.elapsed();
    }
    println!("{:?}", tmp.div_f32(100.0));

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
