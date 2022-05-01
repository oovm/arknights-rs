mod stars;

/// 计算干员升级所需的所有数据
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LevelUpCostDB {
    /// 精 0 每升一级需要消耗的资源
    pub stage0: Vec<usize>,
    /// 精 1 需要的资源
    pub elite1: usize,
    /// 精 1 每升一级需要消耗的资源
    pub stage1: Vec<usize>,
    /// 精 2 需要的资源
    pub elite2: usize,
    /// 精 2 每升一级需要消耗的资源
    pub stage2: Vec<usize>,
}

/// 干员升级所需要的资源
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LevelUpCost {
    /// 精 0 阶段升级需要消耗的资源
    pub stage0: usize,
    /// 精 1 阶段升级需要消耗的资源
    pub stage1: usize,
    /// 精 2 阶段升级需要消耗的资源
    pub stage2: usize,
    /// 精 1 消耗的资源
    pub elite1: usize,
    /// 精 2 消耗的资源
    pub elite2: usize,
}

impl LevelUpCostDB {
    /// 该干员是否能精1
    pub fn can_elite1(&self) -> bool {
        !self.stage1.is_empty()
    }
    /// 该干员是否能精2
    pub fn can_elite2(&self) -> bool {
        !self.stage2.is_empty()
    }
    /// 该干员各阶段的最大等级
    pub fn max_level(&self) -> [usize; 3] {
        [self.stage0.len(), self.stage1.len(), self.stage2.len()]
    }
    /// 需要花费的资源量
    pub fn cost(&self, from: (usize, usize), to: (usize, usize)) -> Result<usize, String> {
        self.cost_detail(from, to).map(|cost| cost.sum())
    }
    /// 列出每个阶段需要花费的资源量
    ///
    /// ## 带有容错机制
    /// - 如果等级是 0, 视为 1 级
    /// - 如果等级上限超出实际上限, 超出部分填 0, 不影响结果
    pub fn cost_detail(&self, from: (usize, usize), to: (usize, usize)) -> Result<LevelUpCost, String> {
        if to.0 < from.0 {
            return Err(format!("目标精英化阶段 `{}` 比其实精英化阶段 `{}` 低", to.0, from.0));
        }
        if to.0 > 2 {
            return Err(format!("目标精英化阶段 `{}` 非法", to.0));
        }
        if to.1 < from.1 {
            return Err(format!("目标等级 `{}` 比起始等级 `{}` 低", to.1, from.1));
        }
        let mut cost = LevelUpCost {
            stage0: 0,
            stage1: 0,
            stage2: 0,
            //
            elite1: self.elite1 as usize,
            elite2: self.elite2 as usize,
        };
        let delta = to.1 - from.1;
        let from_index = from.1.saturating_sub(1);
        let till_index = to.1.saturating_sub(1);
        match (from.0, to.0) {
            (0, 0) => {
                cost.stage0 += self.stage0.iter().skip(from_index).take(delta).cloned().sum::<usize>();
            }
            (0, 1) => {
                cost.stage0 += self.stage0.iter().skip(from_index).cloned().sum::<usize>();
                cost.stage1 += self.stage1.iter().take(till_index).cloned().sum::<usize>();
            }
            (0, 2) => {
                cost.stage0 += self.stage0.iter().skip(from_index).cloned().sum::<usize>();
                cost.stage1 += self.stage1.iter().cloned().sum::<usize>();
                cost.stage2 += self.stage2.iter().take(till_index).cloned().sum::<usize>();
            }
            (1, 1) => {
                cost.stage1 += self.stage1.iter().skip(from_index).take(delta).cloned().sum::<usize>();
            }
            (1, 2) => {
                cost.stage1 += self.stage1.iter().skip(from_index).cloned().sum::<usize>();
                cost.stage2 += self.stage2.iter().take(till_index).cloned().sum::<usize>();
            }
            (2, 2) => {
                cost.stage2 += self.stage2.iter().skip(from_index).take(till_index).cloned().sum::<usize>();
            }
            _ => unreachable!(),
        }
        Ok(cost)
    }
}

impl LevelUpCost {
    /// 求出需要的资源总量
    pub fn sum(&self) -> usize {
        self.stage0 + self.stage1 + self.stage2 + self.elite1 + self.elite2
    }
}
