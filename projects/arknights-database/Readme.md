# 明日方舟数据库

## 升级计算

```rust
#[test]
fn test_cost() {
    println!("6 星从 001 升到 210 需要:");
    let model = LevelUpCostDB::star6_cash();
    let exp = model.cost_detail((0, 01), (2, 10)).unwrap();
    println!("龙门币: {:?}", exp);
    let model = LevelUpCostDB::star6_exp();
    let exp = model.cost_detail((0, 01), (2, 10)).unwrap();
    println!("经验值: {:?}", exp);
}
```

```text
6 星从 001 升到 210 需要:
龙门币: 592408
经验值: 367151
```

## 理智计算

```rust
#[test]
fn test_ap() {
    let model = DoctorLevelDB::default();
    let level = 20;
    let exp = model.cost(1, level).unwrap();
    println!("博士从 1 级升到 {} 级需要 {} 经验", level, exp);
    let time = model.action_point_regen_time(level, 0);
    println!("{} 级博士从 0 回满理智需要 {:?}", level, time);
}
```

```text
博士从 1 级升到 20 级需要 33260 经验
20 级博士从 0 回满理智需要 37800s
```