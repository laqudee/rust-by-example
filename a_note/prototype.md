# 属性
- 属性是应用于某些模块、crate或项的元数据metadata，这些元数据可以用来：
  - 条件编译代码
  - 设置crate名称、版本、类型（二进制文件或库）
  - 禁用lint（警告）
  - 启用编译器的特性（宏、全局导入 glob import）
  - 链接到一个非Rust语言的库
  - 标记函数作为单元测试
  - 标记函数作为基准测试的某个部分

- `#![crate_attribute]`

- `#[item_attribute]`

- `#[attribute = "value"]`
- `#[attribute(key = "value")]`
- `#[attribute(value)]`
- `#[attribute(value, value2)]`
- `#[attribute(v1.v2,v3,v4)]`

# 死代码

```rs
#[allow(dead_code)]
fn unused_function() {}
```

# crate

- crate_type属性可以告知编译器crate是一个二进制的可执行文件还是库

```rs
// 这个 crate 是一个库文件
#![crate_type = "lib"]
// 库的名称为 “rary”
#![crate_name = "rary"]

pub fn public_function() {
    println!("called rary's `public_function()`");
}

fn private_function() {
    println!("called rary's `private_function()`");
}

pub fn indirect_access() {
    print!("called rary's `indirect_access()`, that\n> ");

    private_function();
}

```

```shell
rustc lib.rs
ls lib*
```

# cfg
- 条件编译可能通过两种不同的操作符实现：
  - cfg属性：在属性位置中使用`#cfg[(...)]`
  - cfg!宏：在布尔表达式中使用cfg!(...)

# 自定义条件

- 有部分条件如 target_os 是由 rustc 隐式地提供的，但是自定义条件必须使用 --cfg 标记来传给 rustc

```rs
#[cfg(some_condition)]
fn conditional_function() {
    println!("自定义条件编译");
}

fn main() {
    conditional_function()
}
```