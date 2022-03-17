use std::collections::HashMap;


/// 两位饱和计数器
pub struct Counter(u8);
/// 哈希表，使用 pc 索引获取两位饱和计数器
pub struct BranchHistoryTable(HashMap<usize, Counter>);

/// 简单分支预测器
pub struct BimodalBranchPredictor {
    /// 分支历史表
    bht: BranchHistoryTable,
    /// 分支的数目
    num: usize,
    /// 分支预测错误的数目
    error: usize
}

impl BimodalBranchPredictor {
    pub fn new() -> Self {
        Self {
            bht: BranchHistoryTable(HashMap::new()),
            num: 0,
            error: 0
        }
    }

    pub fn predict(&mut self, pc: usize, res: bool) {
        self.num += 1;
        let mut predict_res: bool = false;
        let mut counter: u8 = 0;
        // 获取分支预测的结果
        if let Some(predict_counter) = self.bht.0.get(&pc) {
            counter = predict_counter.0;
            match predict_counter.0 {
                0 | 1 => { predict_res = false },
                2 | 3 => { predict_res = true },
                _ => { panic!("[Error] Invalid counter") }
            }
        }else {
            let _ = self.bht.0.insert(pc, Counter(1));
            counter = 1;
            predict_res = false;
        }
        if res != predict_res { self.error += 1; }
        if res { 
            if counter <= 2 { counter += 1}
        }else {
            if counter >= 1 { counter -= 1; }
        }
        self.bht.0.insert(pc, Counter(counter)).unwrap();
    }

    pub fn print_res(&self) {
        println!("[Debug] 预测数目: {}, 预测失败数目: {}, 预测错误率: {}", self.num, self.error, self.error as f64  / self.num as f64);
    }
}
