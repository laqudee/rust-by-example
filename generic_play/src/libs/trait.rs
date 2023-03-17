// 不可复制的类型
struct Empty;
struct Null;

// T的泛型trait
trait DoubleDrop<T> {
    // 定义一个调用者的方法，接受一额外的参数T，但不对它做任何事
    fn double_drop(self, _: T);
}

// 对泛型的调用者类型T和任意泛型类型T实现DoubleDrop<T>
impl<T, U> DoubleDrop<T> for U {
    // 次方法获得两个穿入参数的所有权，并释放它们
    fn double_drop(self, _: T) {}
}

pub fn play() {
    let empty = Empty;
    let null = Null;

    empty.double_drop(null);

    // empty;
    // null;
}
