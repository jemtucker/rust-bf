use std;
use std::io::Read;

const MEMORY: usize = 30000;

pub struct Interpreter {
    ip: usize,
    dp: usize,
    data: [u8; MEMORY],
    prog: Vec<char>
}

impl Interpreter {
    pub fn new(p: Vec<char>) -> Interpreter {
        Interpreter { ip: 0, dp: 0, data: [0; MEMORY], prog: p }
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
                '[' => {} ,
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

    fn jump_back(&mut self) {
        if !self.zero() {
            loop {
                if self.prog[self.ip] == '[' {
                    break;
                }

                if self.ip == 0 {
                    panic!("Jump can't find [");
                }

                self.ip -= 1;
            }
        }
    }

    fn zero(&mut self) -> bool {
        self.data[self.dp] == 0
    }
}