extern crate getopts;

mod interpreter;

use getopts::Options;
use interpreter::Interpreter;

use std::fs::File;
use std::io::Read;
use std::io::Error;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let mut opts = Options::new();
    opts.optopt("s", "source", "Input source file", "PATH");
    opts.optflag("d", "debug-mode", "Run in debug mode");
    opts.optflag("h", "help", "Print this help menu");

    let parsed = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(e) => panic!(e.to_string())
    };

    if parsed.opt_str("s").is_some() {
        let debug = parsed.opt_present("d");
        let file = parsed.opt_str("s").unwrap();
        run(&file, debug);
    } else {
        print_usage(&args[0], opts);
        return;
    }
}

fn run(file: &str, debug: bool) {
    match load_file(file) {
        Ok(prog) => {
            let mut int = Interpreter::new(prog);
            int.set_debug(debug);
            int.run();
        }
        Err(e) => {
            println!("Failed to load file {:?}", e);
        }
    }
}

fn load_file(path: &str) -> Result<Vec<char>, Error> {
    let mut file = try!(File::open(path));

    let mut string = String::new();

    try!(file.read_to_string(&mut string));

    let mut prog: Vec<char> = Vec::new();

    for c in string.chars() {
        prog.push(c);
    }

    Ok(prog)
}

fn print_usage(program: &str, opts: Options) {
    let breif = format!("Usage: {} [hs[d]]", program);
    println!("{}", opts.usage(&breif));
}