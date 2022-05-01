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