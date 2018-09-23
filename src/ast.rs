//extern crate nom;
use std::io;
use std::io::Read;
use std::str::Chars;

#[derive(Debug)]
pub enum AST {
    Incr,
    Decr,
    Left,
    Right,
    Read,
    Write,
    Loop(Vec<AST>),
}

pub fn parse(code: &mut Chars) -> Option<Vec<AST>> {
    parseLoop(code, false)
}

fn parseLoop(code: &mut Chars, isLoop: bool) -> Option<Vec<AST>> {
    let mut acc = Vec::new();
    loop {
        let c = match code.next() {
            Some(c) => c,
            None => break,
        };
        match c {
            '+' => acc.push(AST::Incr),
            '-' => acc.push(AST::Decr),
            '>' => acc.push(AST::Right),
            '<' => acc.push(AST::Left),
            ',' => acc.push(AST::Read),
            '.' => acc.push(AST::Write),
            '[' => match parseLoop(code, true) {
                Some(l) => acc.push(AST::Loop(l)),
                None => return None,
            },
            ']' => if isLoop {
                return (Some(acc));
            } else {
                return None;
            },
            ' ' | '\n' => (),
            _ => return None,
        }
    }
    if isLoop {
        None // unterminated loops are invalid
    } else {
        Some(acc)
    }
}

pub fn executeWithVec(code: &Vec<AST>, data: &mut Vec<u8>, mut pointer: usize) -> Option<usize> {
    let stdin = io::stdin();
    let mut bytes = stdin.bytes();
    for instruction in code {
        //println!("current instruction: {:?}, pointer: {:?}, byte at pointer: {:?}", instruction, pointer, data[pointer]);
        match instruction {
            AST::Incr => data[pointer] += 1,
            AST::Decr => data[pointer] -= 1,
            AST::Left => if pointer > 0 {
                pointer -= 1
            } else {
                return None;
            },
            AST::Right => if pointer < data.len() - 1 {
                pointer += 1
            } else {
                return None;
            },
            AST::Read => match bytes.next() {
                Some(Ok(c)) => data[pointer] = c,
                None | Some(Err(_)) => return None,
            },
            AST::Write => print!("{}", char::from(data[pointer])),
            AST::Loop(loopCode) => if data[pointer] != 0 {
                loop {
                    //println!("starting loop iteration.");
                    match executeWithVec(&loopCode, data, pointer) {
                        Some(p) => pointer = p, // advance the pointer to wherever it was after the loop finished
                        None => return None,
                    }
                    //println!("finished loop iteration. pointer: {:?}, byte at pointer: {:?}", pointer, data[pointer]);
                    if data[pointer] == 0 {
                        break;
                    }
                }
            },
        }
    }
    Some(pointer)
}
