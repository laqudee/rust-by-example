# 散列表HashMap

- vector通过整型下标来存储值，HashMap通过键key来存储值

- HashMap的键可以是布尔型、整型、字符串、或任意实现了Eq和 Hash trait的其他类型

- HashMap也是可增长的

- HashMap::with_capacity(unit)创建具有一定初始容量的HashMap
- HashMap::new()创建一个带有默认初始容量的HashMap

# 更改或自定义关键字类型
-   任何实现了Eq和Hash trait的类型都可以充当HashMap的键
    - bool
    - int
    - String，&str
- 注意到 f32 和 f64 没有实现 Hash，这很大程度上是由于若使用浮点数作为 散列表的键，浮点精度误差会很容易导致错误

- 对于所有的集合类，如果他们包含的类型都分别实现了Eq和Hash，那么这些集合类也就实现了 Eq 和 Hash
  - 例如，若T实现了Hash，则Vec<T>也实现了Hash

- 对自定义类型可以轻松地实现 Eq 和 Hash，只需加上一行代码：#[derive(PartialEq, Eq, Hash)]

# 散列集HashSet

- 把HashSet当成这样一个HashMap：只关心其中的键而非值

- HashSet保证不会出现重复元素

- 如果插入的值已经存在，则替换之

- 集合set拥有4种基本操作（全部返回一个迭代器）:
  - union，并集，获得两个集合的所有元素，不含重复值
  - difference，差集，获取属于第一个集合而不属于第二个集合的所有元素
  - intersection，交集，获取同时属于两个集合的所有元素
  - symmetric_difference，对称差，获取所有只属于其中一个集合，而不同时属于两个集合的所有元素

# 引用计数Rc

- 当需要多个所有权时，可以使用Rc引用计数

- 克隆Rc从不执行深拷贝，克隆只创建另一个指向包裹值的指针，并增加计数

# 共享引用计数Arc

- 当线程之间所有权需要共享时，可以使用Arc

- 这个结构通过 Clone 实现可以为内存堆中的值的位置创建一个引用指针，同时增加引用计数器。由于它在线程之间共享所有权，因此当指向某个值的最后一个引用指针退出作用域时，该变量将被删除