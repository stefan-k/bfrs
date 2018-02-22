// Copyright 2018 Stefan Kroboth
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! A basic, certainly not optimized, Brainfuck interpreter.

use std::collections::VecDeque;
use std::num::Wrapping;

/// Holds the state of the interpreter
struct State {
    /// Current position in the buffer
    pos: usize,
}

impl State {
    /// Constructor
    pub fn new() -> Self {
        State { pos: 0 }
    }
}

impl Default for State {
    /// Default
    fn default() -> Self {
        State::new()
    }
}

fn main() {
    // Hello World
    let prog = "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.".as_bytes();
    // let prog = "++[->+<]".as_bytes();
    // The buffer
    let mut buf: VecDeque<Wrapping<u8>> = VecDeque::new();
    // Start with a single `0` in the buffer. `Wrapping` is needed because we want to allow for
    // overflows.
    buf.push_back(Wrapping(0));
    // State holds the position of the pointer
    let mut state = State::new();
    // a stack needed for matching `[` and `]`
    let mut ls: Vec<usize> = vec![];
    // length of the program.
    let plen = prog.len();
    let mut idx = 0;
    loop {
        // Get the current instruction.
        match *prog.get(idx).unwrap() as char {
            // Move right
            '>' => {
                state.pos += 1;
                match buf.get(state.pos) {
                    // The buffer is not empty at the current position.
                    Some(_) => {}
                    // We have exceeded the buffer and need to add another element
                    None => buf.push_back(Wrapping(0)),
                };
            }
            // Move left
            '<' => match state.pos {
                // We are already at the beginning of the buffer, so we will just push to the
                // front. Decreasing `state.pos` is not necessary.
                0 => {
                    buf.push_front(Wrapping(0));
                }
                // Just move the pointer to the left
                _ => state.pos -= 1,
            },
            // Increase the value at the current buffer position. Allow for buffer overflows!
            '+' => {
                if let Some(elem) = buf.get_mut(state.pos) {
                    *elem += Wrapping(1);
                }
            }
            // Decrease the value at the current buffer position. Allow for buffer overflows!
            '-' => {
                if let Some(elem) = buf.get_mut(state.pos) {
                    *elem -= Wrapping(1);
                }
            }
            // Print the `char` at the current buffer.
            '.' => print!("{}", buf.get(state.pos).unwrap().0 as char),
            // We found a `[` which indicates the start of a loop
            '[' => match buf.get(state.pos) {
                // Value at current buffer is `0`, therefore we jump to the position after the
                // matching `]`.
                Some(&Wrapping(0)) => {
                    // Keep track of all the other matching `[` and `]` that we encounter along the
                    // way.
                    let mut lec: usize = 0;
                    // Move forward in the program.
                    loop {
                        idx += 1;
                        match *prog.get(idx).unwrap() as char {
                            // We have found another `[`
                            '[' => lec += 1,
                            // We have found another `]` which does not match the one we are
                            // acually looking for.
                            ']' if lec > 0 => lec -= 1,
                            // We have found the matching `]`
                            ']' if lec == 0 => break,
                            // Ignore anything else
                            _ => {}
                        };
                    }
                }
                // The value in the buffer is nonzero, therefore we just push the position of `[`
                // to the ls stack and move on.
                _ => {
                    ls.push(idx);
                }
            },
            // We found a `]` which indicates the end of a loop
            ']' => {
                match buf.get(state.pos) {
                    // If the value in the buffer at the current position is nonzero, we move to
                    // the matching `[` which we have kept track of in the `ls` stack.
                    Some(c) if (*c).0 != 0 => {
                        idx = *ls.get(ls.len() - 1).unwrap();
                    }
                    // The value at the current position of the buffer is zero, therefore we pop
                    // the matching `[` off the `ls` stack and move on.
                    _ => {
                        ls.pop();
                        ()
                    }
                };
            }
            // Match any other character
            _ => {}
        };
        // Move to the next instruction, break if end of program is reached.
        idx += 1;
        if idx >= plen {
            break;
        }
    }
    // Print the final buffer
    println!("Buffer: {:?}", buf);
}
