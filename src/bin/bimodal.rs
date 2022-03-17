use std::fs::File;
use std::io::{BufReader, BufRead};
use std::env;
use regex::Regex;
use bp_emulator::{ BimodalBranchPredictor, Predictor };

fn main() {
    let args = env::args().collect::<Vec<String>>();
    if args.len() < 3 {
        panic!("[Error] Least three arguments")
    }
    let m = usize::from_str_radix(args[1].as_str(), 16).unwrap();
    let trace = &args[2];
    
    let mut bp = BimodalBranchPredictor::new(m);
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
    let mut output = File::create("bimodal.txt").unwrap();
    bp.output(&mut output);
}