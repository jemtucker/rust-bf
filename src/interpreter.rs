use std;
use std::io::Read;

const MEMORY: usize = 20;

pub struct Interpreter {
    ip: usize,          // Instruction pointer
    dp: usize,          // Data pointer
    data: [u8; MEMORY], // Data array
    prog: Vec<char>,    // Program to interpret
    lps: Vec<usize>,    // Loop pointer stack
    debug: bool         // Debug mode
}

impl Interpreter {
    pub fn new(p: Vec<char>) -> Interpreter {
        Interpreter { 
            ip: 0, 
            dp: 0, 
            data: [0; MEMORY], 
            prog: p, 
            lps: Vec::new(),
            debug: false 
        }
    }

    pub fn set_debug(&mut self, debug: bool) {
        self.debug = debug;
    }

    pub fn run(&mut self) {
        loop {

            if self.prog.len() == self.ip {
                break;
            }

            if self.debug {
                self.debug_print();
            }
            
            match self.prog[self.ip] {
                '>' => self.dp = self.dp.wrapping_add(1),
                '<' => self.dp = self.dp.wrapping_sub(1),
                '+' => self.data[self.dp] = self.data[self.dp].wrapping_add(1),
                '-' => self.data[self.dp] = self.data[self.dp].wrapping_sub(1),
                '.' => self.print_byte(),
                ',' => self.read_byte(),
                '[' => self.jump_fwd(),
                ']' => self.jump_back(),
                _ => {} 
            }

            self.ip += 1;
        }
    }

    fn print_byte(&mut self) {
        print!("{}", self.data[self.dp] as char);
    }

    fn read_byte(&mut self) {
        for byte in std::io::stdin().bytes() {
            match byte {
                Ok(b) => self.data[self.dp] = b,
                Err(x) => panic!("{:?}", x)
            }

            break;
        }
    }

    fn jump_fwd(&mut self) {
        if self.zero() {
            let mut nested = 0;

            loop {
                self.ip += 1;

                if self.ip == self.prog.len() {
                    panic!("Missing jump target (]) : Instruction {}", self.ip);
                }

                // Count the number of nested loops to ensure we jump to 
                // the matching end tag.
                match self.prog[self.ip] {
                    ']' => if nested == 0 { break; } else { nested -= 1 },
                    '[' => nested += 1,
                    _ => {}
                }
            }
        } else {
            // Push pointer onto the stack
            self.lps.push(self.ip - 1);
        }
    }

    fn jump_back(&mut self) {
        let top = self.lps.pop();
        if !self.zero() {
            match top {
                Some(p) => self.ip = p,
                None    => panic!("Missing jump target ([) : Instruction {}", self.ip)
            }
        }
    }

    fn zero(&mut self) -> bool {
        self.data[self.dp] == 0
    }

    fn debug_print(&self) {
        let instr = self.prog[self.ip];

        match instr {
            '>' | '<' | '+' | '-' | '.' | ',' | '[' | ']' => {
                print!("{} ({}) ", self.ip, instr);
                print!("*{}* ", self.dp);
                print!("{:?} ", self.data);
                println!("{:?}", self.lps);
            },
            _ => {}
        }
    } 
}