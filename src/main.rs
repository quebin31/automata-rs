mod automata;
mod parser;

use parser::{AutomataParser, FileParser};
use std::env::args;
use std::fs::write;

fn main() {
    let args: Vec<_> = args().collect();
    if args.len() != 3 {
        println!("Usage: {} <input> <output>", args[0]);
        panic!("Bad usage!");
    }

    let input_file = &args[1];
    let output_file = &args[2];

    let parser = FileParser::with_filename(input_file);
    let automata = parser.parse();

    let automata = automata.to_deterministic();
    write(output_file, &format!("{}", automata)).expect("Failed to write");
}
