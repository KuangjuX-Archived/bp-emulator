use std::{collections::HashMap, io::Write};
use bit_field::BitField;
use std::fs::File;

pub trait Predictor {
    fn predict(&mut self, pc: usize, is_jump: bool); 
    fn num(&self) -> usize;
    fn error(&self) -> usize;
    fn bht(&self) -> &BranchHistoryTable;
    fn bits(&self) -> usize;
    fn output(&self, file: &mut File) {
        file.write(format!("number of predictions: {}\n", self.num()).as_bytes()).unwrap();
        file.write((format!("number of mispredictions: {}\n", self.error())).as_bytes()).unwrap();
        file.write(format!("misprediction rate: {}\n", self.error() as f32 / self.num() as f32).as_bytes()).unwrap();
        
        for pc in 0..(1 << self.bits()) as usize {
            let mut s = String::new();
            if let Some(counter) = self.bht().0.get(&pc) {
                s = format!("{} {}\n", pc, counter.0);
            }else{
                s = format!("{} {}\n", pc, 1);
            }
            file.write(s.as_bytes()).unwrap();
        }
    }
}


/// 两位饱和计数器
pub struct Counter(u8);
/// 哈希表，使用 pc 索引获取两位饱和计数器
pub struct BranchHistoryTable(HashMap<usize, Counter>);

/// 简单分支预测器
pub struct BimodalBranchPredictor {
    /// 表示使用 pc 的哪几位进行 Hash
    m: usize,
    /// 分支历史表
    bht: BranchHistoryTable,
    /// 分支的数目
    num: usize,
    /// 分支预测错误的数目
    error: usize
}

impl BimodalBranchPredictor {
    pub fn new(m: usize) -> Self {
        Self {
            m: m,
            bht: BranchHistoryTable(HashMap::new()),
            num: 0,
            error: 0
        }
    }
}

impl Predictor for BimodalBranchPredictor {
    fn predict(&mut self, pc: usize, is_jump: bool) {
        let bits = self.m + 2;
        let select_bits = pc.get_bits(2..bits);
        self.num += 1;
        let mut predict_jump: bool = false;
        let mut counter: u8 = 0;
        // 获取分支预测的结果
        if let Some(predict_counter) = self.bht.0.get(&select_bits) {
            counter = predict_counter.0;
            match predict_counter.0 {
                0 | 1 => { predict_jump = false },
                2 | 3 => { predict_jump = true },
                _ => { panic!("[Error] Invalid counter") }
            }
        }else {
            let _ = self.bht.0.insert(select_bits, Counter(1));
            counter = 1;
            predict_jump = false;
        }
        if is_jump != predict_jump { self.error += 1; }
        if is_jump { 
            if counter <= 2 { counter += 1 }
        }else {
            if counter >= 1 { counter -= 1 }
        }
        // println!("[Debug] counter: {}", counter);
        self.bht.0.insert(select_bits, Counter(counter)).unwrap();
    }
    fn num(&self) -> usize { self.num }

    fn error(&self) -> usize { self.error }

    fn bht(&self) -> &BranchHistoryTable { &self.bht }

    fn bits(&self) -> usize { self.m }

}

pub struct GShareBranchPredictor {
    /// n 位全局分支历史寄存器
    gbhr: usize,
    /// 分支历史表
    bht: BranchHistoryTable,
    n: usize,
    m: usize,
    /// 分支的数目
    num: usize,
    /// 分支错误的数目
    error: usize
}

impl GShareBranchPredictor {
    pub fn new(m: usize, n: usize) -> Self {
        Self {
            gbhr: 0,
            bht: BranchHistoryTable(HashMap::new()),
            n: n,
            m: m,
            num: 0,
            error: 0
        }
    }
}

impl Predictor for GShareBranchPredictor {
    fn predict(&mut self, pc: usize, is_jump: bool) {
        let gbhr = self.gbhr.get_bits(0..self.n);
        let bits = self.m + 2;
        let pc = pc.get_bits(2..bits);
        let xor_bits = pc.get_bits(self.m - self.n..self.m) ^ gbhr;
        let select_bits = (xor_bits << (self.m - self.n)) | pc.get_bits(0..self.m - self.n);
        self.num += 1;
        let mut predict_jump: bool = false;
        let mut counter: u8 = 0;
        if let Some(predict_counter) = self.bht.0.get(&select_bits) {
            counter = predict_counter.0;
            match predict_counter.0 {
                0 | 1 => { predict_jump = false },
                2 | 3 => { predict_jump = true },
                _ => { panic!("[Error] Invalid counter") }
            }
        }else {
            let _ = self.bht.0.insert(select_bits, Counter(1));
            counter = 1;
            predict_jump = false;
        }
        if is_jump != predict_jump { self.error += 1; }
        if is_jump { 
            if counter <= 2 { counter += 1 }
            self.gbhr = (self.gbhr << 1) + 1;
        }else {
            if counter >= 1 { counter -= 1 }
            self.gbhr = self.gbhr << 1;
        }
        self.bht.0.insert(select_bits, Counter(counter)).unwrap();
    }

    fn num(&self) -> usize { self.num }

    fn error(&self) -> usize { self.error }

    fn bht(&self) -> &BranchHistoryTable { &self.bht }

    fn bits(&self) -> usize { self.m }

}
