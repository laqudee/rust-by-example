# 测试

- 单元测试
- 文档测试
- 集成测试

- rust也支持在测试中指定额外的依赖
  - 开发依赖

- `[dev-dependencies]`

```rs
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq; // 仅用于测试, 不能在非测试代码中使用

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
    }
}

```