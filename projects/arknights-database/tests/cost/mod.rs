use super::*;

#[test]
fn test_star_normal_cost() {
    println!("6 星从 001 升到 210 需要:");
    let model = LevelUpCostDB::star6_cash();
    let exp = model.cost_detail((0, 01), (2, 10)).unwrap();
    println!("龙门币: {:?}", exp.sum());
    let model = LevelUpCostDB::star6_exp();
    let exp = model.cost_detail((0, 01), (2, 10)).unwrap();
    println!("经验值: {:?}", exp.sum());
}
