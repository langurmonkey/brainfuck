
// Very simple Brainfuck interpreter.
// MIT license.

use std::{io, fs};
use io::{BufRead, prelude::*};
use std::path::Path;
use std::num::Wrapping;

use clap::{Parser};

const MEM_SIZE: usize = 40_000;

#[derive(Parser, Default)]
#[command(name = "brainfuck")]
#[command(author = "Toni Sagristà Sellés <me@tonisagrista.com>")]
#[command(about = "Brainfuck interpreter written in Rust. Pass it the code or a file to interpret. If there are no arguments, it reads from the standard input.", long_about = None)]
struct CLIArgs {
    /// Program code or file to run.
    input: Option<String>,

    /// Debug mode. Pause after every instruction and print the internal state.
    #[arg(short, long)]
    debug: bool,
}

/// The machine definition.
struct Machine {
    /// Debug mode.
    debug: bool,
    /// Memory data.
    memory: Vec<Wrapping<u8>>,
    /// Memory pointer.
    ptr: usize,
    /// Stack.
    stack: Vec<usize>,
}
impl Machine {
    /// Creates a new machine.
    fn new(mem_size: usize, debug: bool) -> Machine {
        Machine { debug: debug, memory: vec![Wrapping(0); mem_size], ptr: 0, stack: vec![] }
    }

    /// Interprets the given string on this machine.
    fn interpret(&mut self, program: &str) {
        let prog: Vec<u8> = program.as_bytes().to_vec();


        // Instruction pointer, points to current instruction.
        let mut i: usize = 0;

        while i < prog.len() {
            let mut next: usize = i + 1;
            match prog[i] as char {

                // Move pointer.
                '>' => {
                    if self.ptr < self.memory.len() - 1 {
                        self.ptr += 1;
                    } else {
                        panic!("Memory overflow (pointer={})", self.ptr + 1);
                    }
                },
                '<' => {
                    if self.ptr > 0 {
                        self.ptr -= 1;
                    } else {
                        panic!("Memory overflow: (pointer={})", (self.ptr as i64) - 1);
                    }
                },

                // Modify memory.
                '+' => self.memory[self.ptr] += 1,
                '-' => self.memory[self.ptr] -= 1,

                // Print contents of memory.
                '.' => {
                    let contents: u8 = self.memory[self.ptr].0; 
                    print!("{}", contents as char);
                },

                // Read from input.
                ',' => self.memory[self.ptr] = Wrapping(self.read_char()),

                // Conditionals.
                '[' => {
                    if self.memory[self.ptr].0 == 0 {
                        // Go to command after next ']'.
                        next = self.matching_bracket(&prog, i + 1) + 1;
                    } else {
                        self.stack.push(i);
                    }
                },
                ']' => {
                    if self.memory[self.ptr].0 != 0 {
                        // Go to command after next ']'.
                        next = *self.stack.last().unwrap() + 1;
                    } else {
                        self.stack.pop();
                    }
                },

                // Debug command.
                '#' => self.print_state(),

                _ => ()
            }

            if self.debug && next < prog.len() {
                println!("\nCurrent: {}, next: {}", prog[i] as char, prog[next] as char);
                self.print_state();
                self.pause();
            }

            i = next;

        }
    }

    fn print_state(&self) {
        println!("Ptr: {0}, value: {1}", self.ptr, self.memory[self.ptr]);
    }

    fn pause(&self) {
        let mut stdin = io::stdin();
        let mut stdout = io::stdout();

        // We want the cursor to stay at the end of the line, so we print without a newline and flush manually.
        write!(stdout, "Press return to continue.").unwrap();
        stdout.flush().unwrap();

        // Read a single byte and discard
        let _ = stdin.read(&mut [0u8]).unwrap();
    }

    /// Reads a character from the standard input and returns it as a byte.
    fn read_char(&self) -> u8 {
        let input: Option<u8> = std::io::stdin()
            .bytes() 
            .next()
            .and_then(|result| result.ok())
            .map(|byte| byte as u8);
        match input {
            Some(c) => return c,
            None => println!("Error reading character"),
        }
        0 as u8
    }

    /// Finds the matching bracket in the given vector, starting at position i.
    fn matching_bracket(&self, prog: &Vec<u8>, i: usize) -> usize {
        let mut counter: usize = 1;
        for j in i..prog.len() {
            let char: char = prog[j] as char;
            match char {
                '[' => counter += 1,
                ']' => counter -= 1,
                _ => ()
            };

            if counter == 0 {
                return j;
            }
        };

        panic!("Matching bracket not found!");
    }
}

fn main() {
    let args = CLIArgs::parse();

    let mut machine = Machine::new(MEM_SIZE, args.debug);

    if let Some(program) = args.input.as_deref() {
        // If program is a file, read it, otherwise, it is already the program code.
        let path = Path::new(program);
        if path.exists() && path.is_file() {
            if args.debug {
                println!("Loading file: {}", path.display());
            }
            match fs::read_to_string(program) {
                Ok(programstr) => machine.interpret(&programstr),
                Err(e) => panic!("Can not read file: {}, {}", program, e),
            };
        } else {
            if args.debug {
                println!("Interpreting: {}", program);
            }
            machine.interpret(program);
        }
    } else {
        // Read from standard input
        let stdin = io::stdin();
        for line in stdin.lock().lines() {
            let l = line.unwrap();
            if args.debug {
                println!("Interpreting line: {}", l.as_str());
            }
            machine.interpret(l.as_str());
        }
    }

}


