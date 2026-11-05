// errors2.rs
//
// 假设我们正在编写一款可以用代币购买物品的游戏。每件物品需要 5 个代币，
// 每次购买还要额外支付 1 个代币的手续费。
// 玩家会输入想购买的物品数量，`total_cost` 函数负责计算总共需要多少代币。
// 不过，玩家输入的数量是一个字符串，他们可能输入任何内容，不一定是数字！
//
// 目前，这个函数完全没有处理解析失败的情况，也没有正确处理解析成功的情况。
// 我们希望实现的行为是：当字符串无法解析为整数时，`parse` 会返回包含
// `ParseIntError` 的错误结果。此时应立即从当前函数返回这个错误，
// 不再继续执行后面的乘法和加法运算。
//
// 至少有两种正确的实现方式，其中一种要简短得多！
//
// 执行 `rustlings hint errors2`，或在监视模式下使用 `hint` 子命令查看提示。

use std::num::ParseIntError;

pub fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let processing_fee = 1;
    let cost_per_item = 5;
    let qty = item_quantity.parse::<i32>()?;

    Ok(qty * cost_per_item + processing_fee)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn item_quantity_is_a_valid_number() {
        assert_eq!(total_cost("34"), Ok(171));
    }

    #[test]
    fn item_quantity_is_an_invalid_number() {
        assert_eq!(
            total_cost("beep boop").unwrap_err().to_string(),
            "invalid digit found in string"
        );
    }
}
