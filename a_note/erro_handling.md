# 错误处理

- 是处理可能失败情况的过程

- 处理错误的方式：
  - 显示panic主要用于测试，以及不可恢复的错误
  - Option类型是为了值是可选的、或者缺少值并不是错误的情况准备的
  - 当错误发生，且应当由调用者处理时，Result。也可以使用unwrap然后使用expect，但是除了在测试或者原型开发中，请不要这样做

# panic
- 打印一个错误消息，开始回退unwind任务，且通常会退出程序

# Option 和 unwrap

- `Option<T>`
  - `Some<T>`
  - None

- 这些选项可以通过 match 显式地处理，或使用 unwrap 隐式地处理。隐式处理要么 返回 Some 内部的元素，要么就 panic


# 使用?解开Option

- 可以使用match，但?更容易

# 组合算子：map

- 组合算子，以模块化的风格来管理控制流

- Option有一个内置方法map()，这个组合算子可用于Some -> Some，None -> None这样的简单映射

# 组合算子：and_then

- map()以链式调用的方式来简化match语句

- and_then()使用Option包裹的值来调用2其输入函数并返回结果。如果Option是None，返回None

# 结果Result

- Result是Option类型的更丰富的版本，，描述的是可能的错误而不是可能的不存在

- `Result<T, E>`
  - OK<T>
  - Err<E>

- unwrap()

# Result的map

- 组合算子map, and_then
- Option 的 map、and_then、以及很多其他组合算子也为 Result 实现 了

# 给Result取别名

- `type AliasedResult<T> = Result<T, ParseIntError>`; 

# 提前返回

- 使用match语句和提前返回early return 的结合

- 如果发生错误，我们可以停止函数的执行然后返回错误

# 引入?
- 当找到一个Err，可以采取两种行动
  1. panic!
  2. 返回它，因为Err就意味着它已经不能被处理了

- ? 几乎就等于一个会返回 Err 而不是 panic 的 unwrap

- try!宏
  - 现在我们推荐使用 ? 运算符，但是 在老代码中仍然会看到 try!
