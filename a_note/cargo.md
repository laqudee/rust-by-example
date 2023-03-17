# cargo

- 依赖管理与crates.io集成
- 方便的单元测试
- 方便的基准测试

# 依赖

```toml
[package]
name = "foo"
version = "0.1.0"
authors = ["mark"]

[dependencies]
clap = "2.27.1" # 来自crates.io
rand = { git = "https://github.com/rust-lang-nursery/rand" } # 来自网上的仓库
bar = { path = "../bar" } #来自本地文件系统的路径
```

# 约定规范

- 默认二进制名称是main，但可以通过将文件放在bin/目录下来天津其他可执行二进制文件

- 为了使得 cargo 编译或运行这个二进制可执行文件而不是默认或其他二进制可执行文件，我们只需给 cargo 增加一个参数 --bin my_other_bin，其中 my_other_bin 是我们想要使用的二进制可执行文件的名称。

# 测试
- tests目录下的每个文件都是一个单独的集成测试

# 构建脚本

```toml
[package]
...
build = "build.rs"
```

- 构建脚本只是另一个rust文件，此文件将在编译包中的任何其他内容之前，优先进行编译和调用，因此，此文件可实现满足crate的先决条件