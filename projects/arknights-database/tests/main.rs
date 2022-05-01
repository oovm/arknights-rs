use arknights::{DoctorLevelDB, LevelUpCostDB};
mod cost;

#[test]
fn ready() {
    println!("it works!")
}

#[test]
fn test_ap() {
    let model = DoctorLevelDB::default();
    let level = 20;
    let exp = model.cost(1, level).unwrap();
    println!("博士从 1 级升到 {} 级需要 {} 经验", level, exp);
    let time = model.action_point_regen_time(level, 0);
    println!("{} 级博士从 0 回满理智需要 {:?}", level, time);
}
