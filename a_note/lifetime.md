# 生命周期

- 编译器中的借用检查器用它来保证所有的借用都是有效的，确切地说，一个变量的生命周期在它创建的时候开始，在他销毁的时候结束

- 虽然生命周期和作用域经常被一起提到，但他们并不相同

- 例如考虑这种情况，我们通过 & 来借用一个变量。该借用拥有一个生命周期，此生命周期由它声明的位置决定。于是，只要该借用在出借者（lender）被销毁前结束，借用就是有效的。然而，借用的作用域则是由使用引用的位置决定的。

- 其实生命周期的提出就是为了解决出借者的生命周期和借用者生命周期不一致的问题

# 显示标注
- rust 需要显式来确定引用的生命周期
- '
- 和闭包类似，使用生命周期需要泛型

```rs
foo<'a>

foo<'a, 'b>
// foo的生命周期不能超出'a 'b的任意一个周期
```

# 函数
- 排除省略elision的情况，带上生命周期的函数签名有一些限制
  - 任何引用都必须拥有标注好的生命周期
  - 任何被返回的引用都必须有和某个输入量相同的生命周期或时静态类型static

- 如果没有输入的函数返回引用，有时会导致返回的引用指向无效数据，这种情况下禁止它返回这样的引用

# 方法

# 结构体
- 在结构体中标注生命周期也和函数一样

# 约束
- 就如泛型类型能够被约束一样
- 生命周期本身就是泛型，也可以使用约束
- :字符的意义在这里稍有不同：
  - T: 'a 在T中的所有引用都必须比生命周期'a活的更长
  - T: Trait + 'a: T类型必须实现Trait trait，并且在T中所有引用都必须比'a活得长

# 强制转换
- 一个较长的生命周期可以强制转换为一个较短的生命周期，使它在一个通常情况下不能工作的作用域内也能工作

- 强制转换可由编译器隐式地推导并执行，也可以通过声明不同的生命周期的形式实现

# static 
- 'static 生命周期是可能的生命周期中做长的，它会在整个程序运行的时期存在

- 可以被强制转换为一个更短的生命周期

- 使用static 声明产生常量
- 产生一个&'static str 类型的string 字面量

# 省略
- 有些生命周期太常用二零，所以借用检查器会隐式地添加他们