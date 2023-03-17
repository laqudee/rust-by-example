# 标准库更多介绍

- 线程Threads
- 信道Channels
- 文件输入输出File I/O

# 线程
- 通过spawn函数提供了创建本地操作系统线程的机制，该函数的参数是一个通过值捕获变量的闭包

# 测试实例： map-reduce

# 通道
- rust为线程之间的通信提供了异步的通道channel。
- 通道允许两个端点之间信息的单向流动：sender和Receiver

# 路径

- Path结构体代表了底层文件系统的文件路径
  - posix::Path，针对UNIX系统
  - windows::Path，针对Window系统

- prelude

- Path可从OsStr类型创建，并且它提供数种方法，用于获取路径指向的文件/目录的信息

- Path在内部并不是用UTF-8字符串表示的，而是存储为若干字节Vec<u8>的vector
  - 因此将Path转换为&str并非零开销，且可能失败，所以Path返回的是option

# 文件输入输出I/O

- File结构体表示一个被打开的文件它包裹了一个文件描述符，并赋予了对所表示的文件的读写能力

- 进行文件I/O操作可能会出现各种错误，因此File的所有方法都返回Result

- 所有 I/O 操作的失败都变成显式的

# 打开文件Open

- open 静态方法能够以只读模式打开一个文件

- File拥有资源，即文件描述符file descriptor，他会在自身被drop时关闭文件

# 创建文件create

- create静态方法以只写模式打开，若文件已经存在，则旧内容将被销毁，否则创建一个新的文件

- open_mode

- append

# 读取行

- 方法lines()在文件的行上返回一个迭代器

- File::open 需要一个泛型 AsRef<Path>。这正是 read_lines() 期望的输入