# 子进程

- process::Output结构体表示已结束的子进程的输出
- process::Command结构体是一个进程创建者process builder

# 管道

- std::Child 结构体代表了一个正在运行的子进程，他暴露了stdin，stdout，stderr句柄，从而可以通过管道与所代表的进程交互

# 等待

- 如果你想等待一个 process::Child 完成，就必须调用 Child::wait，这会返回 一个 process::ExitStatus

# 程序参数
- 标准库
  - 命令行参数可使用std::env::args进行接收，这将返回一个迭代器，该迭代器会对每个参数举出一个字符串

# 外部语言函数接口

- rust提供了到C语言库的外部语言函数接口FFI

- 外部语言必须在extern代码块种声明，且该代码块要带有一个包含库名称 的 #[link] 属性