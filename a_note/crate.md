# crate

- crate，中文包，是rust的编译单元

- 当调用rust some_file.rs时，some_file.rs被当做crate文件

- 若文件中包含mod声明，那么模块文件的内容将在编译之前被插入crate文件的相应声明处，模块不会单独编译，只有crate才会被编译

- crate可以编译成二进制可执行文件binary或库文件library

# 库

- 创建一个库，然后看看如何连接到另一个crate

# 使用库
- 要将一个 crate 链接到上节新建的库，可以使用 rustc 的 --extern 选项。然后将所有的物件导入到与库名相同的模块下。

- 此模块的操作通常与任何其他模块相同。