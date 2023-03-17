# 函数

- 使用fn关键字来声明。函数的参数需要标注类型，函数若有返回值，则类型必须在箭头之后指定

- 函数最后的表达式将作为返回值，也可以在函数内使用return语句提前返回一个值，甚至可以在循环或if内部使用

# 方法

- 方法是依附于对象的函数，这些方法通过关键字self来访问对象中的数据和其他

- 方法在impl代码块中定义

# 闭包

- rust中的闭包closure，也叫lamba表示式，是一类能够捕获周围作用域中变量的函数

- 它们的语法和能力使它们在临时使用时相当方便，调用一个闭包和调用一个函数完全相同，不过调用闭包时，输入和返回类型两者都可以自动推导，而输入变量名必须指明

- 其他特点：
  - 声明时使用||替代()将输入参数括起来
  - 函数体定界符{}对于单个表达式是可选的
  - 有能力捕获外部环境的变量


# 捕获
- 闭包本质上很灵活，能做功能要求的事情，使闭包在没有类型标注的情况下运行
- 这使得捕获能够灵活地适应用例，move，borrow

- 闭包通过以下方式捕获变量
  - 通过引用：&T
  - 通过可变引用： &mut T
  - 通过值： T

- 闭包优先通过引用来捕获变量，并且仅在需要的时候使用其他方式

- 在竖线|之前使用move会强制闭包取得被捕获变量的所有权

# 作为输入参数
- 虽然rust无需说明就能在大多数时候完成变量捕获，在编写函数时，这种模糊是不应该的

- 当以闭包作为输入参数时，必须指出闭包的完整类型，通过使用一下trait中的一种来指定

- 受限制程度的顺序（递减）：
  - Fn：通过&T，引用
  - FnMut：通过&mut T，可变引用
  - FnOnce：通过T，获取变量所有权

- 对闭包所要获取的每个变量，编译器都将以限制最少的方式获取

# 类型匿名
- 使用闭包作为函数参数，这要求闭包是泛型的。

# 输入函数
- 闭包可以作为参数，函数也可以作为输入
- Fn、FnMut 和 FnOnce 这些 trait 明确了闭包如何从周围的作用域中捕获变量

# 作为输出参数

- 闭包作为输入参数是可能的，所以返回闭包作为输出参数也是可能的

- 只能使用impl Trait才能返回一个闭包

- 返回闭包的有效特征
  - Fn
  - FnMut
  - FnOnce

- 还必须使用move关键字，它表明所有的捕获都是通过值进行的。这是必须的，因为在函数退出时，任何通过引用的捕获都被丢弃，在闭包中留下无效的引用。

- Iterator::any

- Iterator::find

# 高阶函数
- HOF，输入一个或一些函数，并且/或者产生一个更有用的函数的函数

- HOF和惰性迭代器给rust带来了函数式编程额风格

# 发散函数

- 发散函数绝不会返回，使用!标记，这是一个空类型

- 与 ()不同

- 这种类型的主要优点是它可以被转换为任何其他类型，从而可以在需要精确类型的地方使用，例如在 match 匹配分支