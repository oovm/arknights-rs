mod levels;
use std::time::Duration;

/// 博士升级需要的所有数据
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DoctorLevelDB {
    /// 经验表
    exp: Vec<usize>,
    /// 理智表
    ap: Vec<usize>,
}

impl DoctorLevelDB {
    /// 升级需要的经验值
    pub fn cost(&self, from: usize, to: usize) -> Result<usize, String> {
        if to < from {
            return Err(format!("目标等级 `{}` 小于起始等级 `{}`", to, from));
        }
        Ok(self.exp.iter().skip(from - 1).take(to - from).cloned().sum())
    }
    /// 给出对应等级的理智上线
    pub fn action_points_max(&self, level: usize) -> usize {
        self.ap[level.saturating_sub(1)]
    }
    /// 理智回满需要的时间(秒)
    pub fn action_point_regen_time(&self, level: usize, current: usize) -> Duration {
        let rest = self.action_points_max(level).saturating_sub(current);
        let time = rest * Self::ACTION_POINT_REGEN * 60;
        Duration::from_secs(time as u64)
    }
}
