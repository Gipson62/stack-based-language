use std::{collections::HashMap, iter::Peekable, vec::IntoIter};

use internment::Intern;

use crate::{
    instructions::Instruction,
    lexer::{Error, Token, TokenKind},
};

pub struct Block {
    id: Intern<String>,
    ins: Vec<Instruction>,
    len: usize,
}

pub struct Parser {
    tokens: Peekable<IntoIter<Token>>,
    blocks: Vec<Block>,
    constants: HashMap<String, i32>,
    pos: usize,
}

impl Parser {
    pub fn parse(tokens: Vec<Token>) -> Result<Vec<Instruction>, Box<dyn Error>> {
        let toks = tokens.into_iter().peekable();
        let mut parser = Parser {
            tokens: toks,
            blocks: Vec::new(),
            constants: HashMap::new(),
            pos: 0,
        };
        match parser.parse_section() {
            Ok(_) => match parser.parse_code() {
                Ok(_) => {}
                Err(e) => {
                    println!("{}", e.message())
                }
            },
            Err(e) => {
                println!("{}", e.message())
            }
        }
        todo!()
    }
}

impl Parser {
    fn parse_section(&mut self) -> Result<(), Box<dyn Error>> {
        if let Some(tok) = self.tokens.next() {
            if tok.kind() == TokenKind::Dot {
                if let Some(tok) = self.tokens.next() {
                    if tok.kind() == TokenKind::Keyword(Intern::new(String::from("section"))) {
                        while let Some(tok) = self.tokens.peek() {
                            if tok.kind() == TokenKind::Dot {
                                break;
                            }
                            match self.parse_const() {
                                Ok((k, v)) => {
                                    self.constants.insert(k, v);
                                }
                                Err(e) => {
                                    if e.recoverable() {
                                        continue;
                                    } else {
                                        return Err(e);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        Ok(())
    }
    fn parse_const(&mut self) -> Result<(String, i32), Box<dyn Error>> {
        todo!()
    }
}

impl Parser {
    fn parse_code(&mut self) -> Result<(), Box<dyn Error>> {
        todo!()
    }
    fn parse_block(&mut self) -> Result<Block, Box<dyn Error>> {
        todo!()
    }
}
