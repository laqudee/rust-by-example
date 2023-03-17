# 格式化输出

- 打印操作一般由`std::fmt`里面所定义的一些列宏决定
    - format!：将格式化文本写到字符串
    - print!：将文本输出到控制台io::stdout
    - println!：追加一个换行符
    - eprint!：将文本输出到标准错误io::stderr
    - eprintln!

- std::fmt包含多种trait来控制文字显示，其中重要的两种trait：
  - fmt::Debug，使用{:?}标记。格式化文本以供调用
  - fmt::Disply，使用{}标记。

# 调试

- 所有类型，若想用std::fmt的格式打印，都要求实现至少一个可打印的traits

- 所有类型都能推到fmt::Debug；但是fmt::Display需要手动实现

- 所有std库类型都天生可以使用{:?}来打印

- `{:#?}`提供了美化打印功能

- 通过手动实现fmt::Display来控制显示效果

# 显示

- fmt::Debug通常看起来不太简洁，因此自定义输出的外观是更可取的，通过手动实现fmt::Display来做到

- fmt::Display采用{}标记

- fmt::Display被实现但是fmt::Binary没有，所以fmt::Binary不能使用

# 测试实例：List
- ?操作符

```rs
// 对write!进行尝试try，观察是否出错。若发生错误，返回相应的错误
// 否则继续执行后面的语句

write!(f, "{}", value)?;

// 老写法
try!(write!(f, "{}", value));
```