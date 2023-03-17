# 流程控制

# if/else
- rust语言中的布尔判断条件不必使用小括号包裹，且每个条件后面跟着一个代码块

# loop循环
- 无限循环
- break跳出循环
- continue跳过循环体的剩余部分开始下一轮循环

# 嵌套循环和标签
- 使用'label标注循环，标签必须传递给break/continue

# 从loop循环中返回

- loop有个用途就是尝试一个操作直到成功返回为止。
- 将值放到break之后，它将会被loop表达式返回

# while
- 条件满足时循环

# for 循环

- for与区间
  - for in 可以遍历一个Iterator迭代器

- 创建迭代器最简单的方法是使用区间标记a..b

- a..=100

- for与迭代器
    - into_iter()，非借用
    - iter()，不可变借用
    - iter_mut()，可变借用

# match 匹配

- rust通过match关键字来提供模式匹配和C语言中的switch用法类似

# match 解构
- 解构元组
- 解构枚举
- 解构指针
- 解构结构体

- 指针和引用
  - 对指针来说，解构和解引用要区分开，因为二者概念不同

- 解引用：*
- 解构使用&、ref、ref mut

- 卫语句 过滤分支

- 绑定
  - 在match中，若间接地访问一个变量，则不经过重新绑定就无法在分支中再使用它，match提供@符号来绑定变量到名称

# if let 
- 一些场合下，match匹配2枚举类型并不优雅
- iflet更简洁

# while let

- 和if let 类似，while let也可以把别扭的match改写的好看一些

