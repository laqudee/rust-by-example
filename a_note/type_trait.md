# 类型转换

- rust使用trait解决类型之间的转换问题

- 最一般的转换会用到From和Into两个trait

- 特殊的trait

# From 和 Into

- 二者内部是相关联的

- From trait允许一种类型定义“怎么根据另一种类型生成自己”

```rs
    let my_str = "hello";
    let my_string = String::from(my_str);
```

- std::convert::From

- Into trait就是把From trait倒过来而已，你的类型实现了From，也就实现了Into trait

- 使用Into trait 通常要求指明要转换的类型，因为编译器大多数时间不能推断它

# TryFrom and TryInto

- 二者是类型转换的通用trait，不同于From/Into的是，这二者用于易出错的转换，返回值都是`Result<T, E>`

# ToString 和 FromStr

- ToString，要把任何类型转换成String，只要实现ToString trait；然而一般不直接这么做，要实现fmt::Display trait 它会自动提供 ToString，并且还可以用来打印类型

- 解析字符串
- 字符串转成数字，标准手段parse函数

- 是要目标类型实现了FromStr trait，就可以用parse函数把字符串转换成目标类型