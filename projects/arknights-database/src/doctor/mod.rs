/// 博士升级需要的所有数据
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DoctorLevelUpDB {}

impl DoctorLevelUpDB {
    ///
    pub fn cost(&self, from: usize, to: usize) -> Result<usize, String> {
        Ok(0)
    }
}
