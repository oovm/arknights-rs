mod stars;

pub struct LevelUpCost {
    pub elite0: Vec<u16>,
    pub elite1: Vec<u16>,
    pub elite2: Vec<u16>,
}

impl LevelUpCost {
    /// 该干员是否能精1
    pub fn can_elite1(&self) -> bool {
        !self.elite1.is_empty()
    }
    /// 该干员是否能精2
    pub fn can_elite2(&self) -> bool {
        !self.elite2.is_empty()
    }
    /// 该干员各阶段的最大等级
    pub fn max_level(&self) -> [usize; 3] {
        [self.elite0.len(), self.elite1.len(), self.elite2.len()]
    }
}
