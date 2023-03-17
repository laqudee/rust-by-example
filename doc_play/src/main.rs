/// 这里给出一个“人”的表示
pub struct Person {
    /// 一个人必须有名字
    name: String,
}

impl Person {
    /// 返回具有指定名字的一个人
    ///
    /// # 参数
    ///
    /// * `name` - 字符串切片，代表人的名字
    ///
    /// # 示例
    ///
    /// ```
    /// // 在文档注释中你可以书写代码块
    /// // 如果向rustdoc传递 --test 参数，他还会帮你测试注释文档中的代码！
    /// use doc::Person;
    /// let person = Person::new("name");
    /// ```
    pub fn new(name: &str) -> Person {
        Person {
            name: name.to_string(),
        }
    }

    pub fn hello(&self) {
        println!("Hello, {}", self.name);
    }
}

fn main() {
    let john = Person::new("John");
    john.hello();
}
