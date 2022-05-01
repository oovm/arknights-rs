/// 干员精英化所需要的资源
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EliteCost {
    /// 精 1 消耗的资源
    pub elite1: Vec<&'static str>,
    /// 精 2 消耗的资源
    pub elite2: Vec<&'static str>,
}
