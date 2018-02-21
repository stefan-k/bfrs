// Copyright 2018 Stefan Kroboth
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! TODO Documentation

use std::collections::VecDeque;
use std::num::Wrapping;

struct State {
    pos: usize,
}

impl State {
    pub fn new() -> Self {
        State { pos: 0 }
    }
}

impl Default for State {
    fn default() -> Self {
        State::new()
    }
}

fn main() {
    let prog = "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.".as_bytes();
    let mut buf: VecDeque<Wrapping<u8>> = VecDeque::new();
    let mut state = State::new();
    let mut loopstarts: Vec<usize> = vec![];
    buf.push_back(Wrapping(0));
    let plen = prog.len();
    let mut idx = 0;
    // for mut idx in 0..(plen - 1) {
    loop {
        // if let Some(c) = prog.get(idx) {
        match *prog.get(idx).unwrap() as char {
            '>' => {
                // print!("{}", c);
                state.pos += 1;
                match buf.get(state.pos) {
                    Some(_) => {}
                    None => buf.push_back(Wrapping(0)),
                };
            }
            '<' => match state.pos {
                0 => {
                    buf.push_front(Wrapping(0));
                }
                _ => {}
            },
            '+' => {
                if let Some(elem) = buf.get_mut(state.pos) {
                    *elem += Wrapping(1);
                }
            }
            '-' => {
                if let Some(elem) = buf.get_mut(state.pos) {
                    *elem -= Wrapping(1);
                }
            }
            '.' => print!("{}", buf.get(state.pos).unwrap().0 as char),
            '[' => match buf.get(state.pos) {
                Some(&Wrapping(0)) => {
                    // println!("fu");
                    let mut lec: usize = 0;
                    loop {
                        idx += 1;
                        match *prog.get(idx).unwrap() as char {
                            '[' => lec += 1,
                            ']' if lec > 0 => lec -= 1,
                            ']' if lec == 0 => break,
                            _ => {}
                        };
                    }
                }
                _ => {
                    // println!("fu2");
                    loopstarts.push(idx);
                    // println!("f: {}: {:?}", idx, loopstarts);
                }
            },
            ']' => {
                match buf.get(state.pos) {
                    Some(c) if (*c).0 != 0 => {
                        println!("{}: {:?}", idx, loopstarts);
                        // idx = loopstarts.pop().unwrap();
                        idx = *loopstarts.get(loopstarts.len() - 1).unwrap();
                        println!("{}", idx);
                    }
                    _ => {
                        // loopstarts.pop();
                        ()
                    }
                };
            }
            _ => {}
        };
        idx += 1;
        if idx >= plen {
            break;
        }
    }
    // })
    println!("{:?}", buf);
}
