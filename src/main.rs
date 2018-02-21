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
    buf.push_back(Wrapping(0));
    let plen = prog.len();
    // prog.chars()
    //     .map(|c| {
    for mut idx in 0..(plen - 1) {
        // print!("{}", c);
        // if let c = prog.get(idx) {
        if let Some(c) = prog.get(idx) {
            match *c as char {
                '>' => {
                    // print!("{}", c);
                    state.pos += 1;
                    match buf.get(state.pos) {
                        Some(_) => {}
                        None => buf.push_back(Wrapping(0)),
                    };
                }
                '<' => {
                    // print!("{}", c);
                    state.pos -= 1;
                    match buf.get(state.pos) {
                        Some(_) => {}
                        None => {
                            buf.push_front(Wrapping(0));
                            state.pos = 0;
                        }
                    };
                }
                '+' => {
                    // print!("{}", c);
                    if let Some(elem) = buf.get_mut(state.pos) {
                        *elem += Wrapping(1);
                    }
                }
                '-' => {
                    // print!("{}", c);
                    if let Some(elem) = buf.get_mut(state.pos) {
                        *elem -= Wrapping(1);
                    }
                }
                '.' => print!("{}", buf.get(state.pos).unwrap().0 as char),
                '[' => print!("{}", c),
                ']' => print!("{}", c),
                _ => {}
            };
        };
    }
    // })
    println!("{:?}", buf);
}
