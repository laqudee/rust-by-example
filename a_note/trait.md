# 特质 trait

- trait是对未知类型Self定义的方法集，该类型也可以访问同一个trait中定义的其他方法

- 对任何数据类型都可以实现trait

# 派生
- 通过`#[dervie]`属性，编译器能够提供某些trait的基本实现，如果需要更复杂的行为，这些trait也可以手动实现
    - 比较trait：Eq, PartialEq, Ord, PartialOrd
    - Clone
    - Copy
    - Hash
    - Default
    - Debug

# 使用dyn 返回trait
- 每当在堆上分配内存时，rust都会尝试尽可能明确，因此，如果你的函数以这种方式返回指向堆的trait指针，则选哟使用dyn关机找你编写返回类型

- `Box<dyn Animal>`

# 运算符重载

- 很多运算符可以通过trait重载，根据不同的输入完成不同的任务
- 运算符就是语法糖

# Drop

- drop，当对象离开作用域时自动调用。

- Drop trait 也可以为任何自定义数据类型手动打印实现

# Iterator
- Iterator trait 用于对集合collection类型比如数组实现迭代器
- next方法

# impl Trait

- 如果函数返回实现了MyTrait 的类型，可以将其返回类型编写为 -> impl MyTrait。这可以大大简化类型签名

- 有些类型无法写出，例如每个闭包都有自己未命名的具体类型，在使用 impl Trait 语法之前，必须在堆上进行分配才能返回闭包。但是现在你可以像下面这样静态地完成所有操作。

# Clone
- 将资源复制一份
- clone trait

# 父trait
- rust没有继承，但是可以将一个trait定义为另一个trait的超集父trait

# 消除重叠trait
- 由于每个 trait 实现都有自己的 impl 块，因此很清楚您要实现哪个 trait 的 get 方法