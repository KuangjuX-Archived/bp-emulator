use std::collections::HashMap;
use bit_field::BitField;

pub trait Predictor {
    fn predict(&mut self, pc: usize, is_jump: bool); 
    fn print_res(&self);
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
        self.bht.0.insert(select_bits, Counter(counter)).unwrap();
    }

    fn print_res(&self) {
        println!("[Debug] 预测数目: {}, 预测失败数目: {}, 预测错误率: {}", self.num, self.error, self.error as f64  / self.num as f64);
    }
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
        let pc = pc.get_bits(2..self.m + 2);
        let xor_bits = pc.get_bits(self.m - self.n..self.m) ^ gbhr;
        let select_bits = (xor_bits << self.m - self.n) | self.gbhr;
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

    fn print_res(&self) {
        println!("[Debug] 预测数目: {}, 预测失败数目: {}, 预测错误率: {}", self.num, self.error, self.error as f64  / self.num as f64);
    }
}
