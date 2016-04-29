mod interpreter;

use interpreter::Interpreter;

fn main() {
    let input = "++++++++++[>+++++++>++++++++++>+++>+<<<<-]>++.>+.+++++++..+++.>++.<<+++++++++++++++.>.+++.------.--------.>+.>.";
    let mut prog: Vec<char> = Vec::new();

    for c in input.to_string().chars() {
        prog.push(c);
    }

    let mut int = Interpreter::new(prog);
    int.run();
}