mod stars;

pub struct LevelUpCostDB {
    /// 精 0 每升一级需要消耗的资源
    pub elite0: Vec<u16>,
    /// 精 1 每升一级需要消耗的资源
    pub elite1: Vec<u16>,
    /// 精 2 每升一级需要消耗的资源
    pub elite2: Vec<u16>,
}

pub struct CharacterCost {
    /// 精 1 消耗的资源
    pub elite1: Vec<&'static str>,
    /// 精 2 消耗的资源
    pub elite2: Vec<&'static str>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct LevelUpCost {
    /// 精 0 阶段升级需要消耗的资源
    pub elite0: usize,
    /// 精 1 阶段升级需要消耗的资源
    pub elite1: usize,
    /// 精 2 阶段升级需要消耗的资源
    pub elite2: usize,
}

impl LevelUpCostDB {
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
    /// 需要花费的资源量
    pub fn cost(&self, from: (usize, usize), to: (usize, usize)) -> Result<LevelUpCost, String> {
        let [max0, max1, max2] = self.max_level();
        if to.0 < from.0 {
            return Err(format!("目标精英化阶段 `{}` 比其实精英化阶段 `{}` 低", to.0, from.0));
        }
        if to.0 > 2 {
            return Err(format!("目标精英化阶段 `{}` 非法", to.0));
        }
        if to.1 < from.1 {
            return Err(format!("目标等级 `{}` 比起始等级 `{}` 低", to.1, from.1));
        }
        let mut cost = LevelUpCost { elite0: 0, elite1: 0, elite2: 0 };
        let delta = to.1 - from.1;
        let from_index = from.1.saturating_sub(1);
        let till_index = to.1.saturating_sub(1);
        match (from.0, to.0) {
            (0, 0) => {
                cost.elite0 += self.elite0.iter().skip(from_index).take(delta).map(|n| *n as usize).sum::<usize>();
            }
            (0, 1) => {
                cost.elite0 += self.elite0.iter().skip(from_index).map(|n| *n as usize).sum::<usize>();
                cost.elite1 += self.elite0.iter().take(till_index).map(|n| *n as usize).sum::<usize>();
            }
            (0, 2) => {
                cost.elite0 += self.elite0.iter().skip(from_index).map(|n| *n as usize).sum::<usize>();
                cost.elite1 += self.elite0.iter().map(|n| *n as usize).sum::<usize>();
                cost.elite2 += self.elite1.iter().take(till_index).map(|n| *n as usize).sum::<usize>();
            }
            (1, 1) => {
                cost.elite1 += self.elite1.iter().skip(from_index).take(delta).map(|n| *n as usize).sum::<usize>();
            }
            (1, 2) => {
                cost.elite1 += self.elite1.iter().skip(from_index).map(|n| *n as usize).sum::<usize>();
                cost.elite2 += self.elite1.iter().take(till_index).map(|n| *n as usize).sum::<usize>();
            }
            (2, 2) => {
                cost.elite2 += self.elite2.iter().skip(from_index).take(till_index).map(|n| *n as usize).sum::<usize>();
            }
            _ => unreachable!(),
        }
        Ok(cost)
    }
}

#[test]
fn test() {
    let model = LevelUpCostDB::star1_exp();
    let exp = model.cost((0, 1), (0, 5)).unwrap();
    println!("{:?}", exp);
}

pub struct Man {
    star: u8,
}

impl Man {}
