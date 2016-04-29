use std;
use std::io::Read;

const MEMORY: usize = 30000;

pub struct Interpreter {
    ip: usize,          // Instruction pointer
    dp: usize,          // Data pointer
    data: [u8; MEMORY], // Data array
    prog: Vec<char>,    // Program to interpret
    lps: Vec<usize>     // Loop pointer stack
}

impl Interpreter {
    pub fn new(p: Vec<char>) -> Interpreter {
        Interpreter { ip: 0, dp: 0, data: [0; MEMORY], prog: p, lps: Vec::new() }
    }

    pub fn run(&mut self) {
        loop {

            if self.prog.len() == self.ip {
                break;
            }

            match self.prog[self.ip] {
                '>' => self.dp += 1,
                '<' => self.dp -= 1,
                '+' => self.data[self.dp] += 1,
                '-' => self.data[self.dp] -= 1,
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
            loop {
                if self.prog[self.ip] == ']' {
                    break;
                }

                if self.ip == 0 {
                    panic!("Missing jump target (])");
                }

                self.ip += 1;
            }
        } else {
            // Push pointer onto the stack
            self.lps.push(self.ip - 1);
        }
    }

    fn jump_back(&mut self) {
        if !self.zero() {
            let top = self.lps.pop();

            match top {
                Some(p) => self.ip = p,
                None    => panic!("Missing jump target ([)")
            }
        }
    }

    fn zero(&mut self) -> bool {
        self.data[self.dp] == 0
    }
}