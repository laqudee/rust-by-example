// 此函数取得堆分配的内存的所有权
fn destroy_box(c: Box<i32>) {
    println!("Destroying a box that contians {}", c);
}

fn mut_play() {
    let immutable_box = Box::new(5u32);

    println!("immutable_box contains {}", immutable_box);

    // 可变性错误
    //*immutable_box = 4;

    // *移动* box，改变所有权（和可变性）
    let mut mutable_box = immutable_box;

    println!("mutable_box contains {}", mutable_box);

    // 修改 box 的内容
    *mutable_box = 4;

    println!("mutable_box now contains {}", mutable_box);
}

fn partial_move() {
    #[derive(Debug)]
    struct Person {
        name: String,
        age: u8,
    }

    let person = Person {
        name: String::from("Alice"),
        age: 18,
    };

    // name从person中移走，但age只是引用
    let Person { name, ref age } = person;

    println!("The person's age is {}", age);
    println!("The person's name is {}", name);

    // 报错！部分移动值的借用：`person` 部分借用产生
    // println!("The person struct is {:?}", person);

    println!("The person's age from person struct is {}", person.age);
}

pub fn play() {
    // 栈分配的整型
    let x = 5u32;

    // 将x复制给y——不存在资源移动
    let y = x;
    println!("x is {}, y is {}", x, y);

    //  a 是一个指向堆分配的整数的指针
    let a = Box::new(5i32);

    println!("a contains: {}", a);

    // 将a移动到b
    let b = a;
    // 把 `a` 的指针地址（而非数据）复制到 `b`。现在两者都指向
    // 同一个堆分配的数据，但是现在是 `b` 拥有它。

    // 报错！`a` 不能访问数据，因为它不再拥有那部分堆上的内存。
    //println!("a contains: {}", a);

    destroy_box(b);

    // println!("b: {}", b)

    mut_play();

    partial_move();
}
