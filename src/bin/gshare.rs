use std::fs::File;
use std::io::{BufReader, BufRead};
use std::env;
use regex::Regex;
use bp_emulator::{ Predictor, GShareBranchPredictor };

fn main() {
    let args = env::args().collect::<Vec<String>>();
    if args.len() < 4 {
        panic!("[Error] Least three arguments")
    }
    let m = usize::from_str_radix(args[1].as_str(), 10).unwrap();
    let n = usize::from_str_radix(args[2].as_str(), 10).unwrap();
    let trace = &args[3];
    
    let mut bp = GShareBranchPredictor::new(m, n);
    let file = File::open(trace).unwrap();
    let reader = BufReader::new(file);
    for line in reader.lines() {
        if let Ok(line) = line {
            let pattern = Regex::new(r"([0-9a-fA-F]+) ([a-zA-Z])").unwrap();
            let cap = pattern.captures(&line).unwrap();
            let pc = usize::from_str_radix(&cap[1], 16).unwrap();
            let res = match &cap[2] {
                "t" => { true },
                "n" => { false },
                _ => panic!("[Error] Invalid result")
            };
            bp.predict(pc, res);
        }
    }
    let mut output = File::create("gshare_1.txt").unwrap();
    bp.output(&mut output);
}