use std::fs::File;
use std::io::{BufReader, BufRead};
use regex::Regex;
use bp_emulator::BimodalBranchPredictor;

fn main() {
    let mut bp = BimodalBranchPredictor::new();
    let file = File::open("traces/gcc_trace.txt").unwrap();
    let reader = BufReader::new(file);
    let mut line_counts = 0;
    for line in reader.lines() {
        if let Ok(line) = line {
            line_counts += 1;
            let pattern = Regex::new(r"([0-9a-fA-F]+) ([a-zA-Z])").unwrap();
            let cap = pattern.captures(&line).unwrap();
            let pc = usize::from_str_radix(&cap[1], 16).unwrap();
            let res = match &cap[2] {
                "t" => { true },
                "n" => { false },
                _ => panic!("[Error] Invalid result")
            };
            bp.predict(pc, res);
            if line_counts >= 2000 {
                break;
            }
        }
    }
    bp.print_res();
}