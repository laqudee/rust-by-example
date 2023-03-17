pub fn it_work() {
    fn function(i: i32) -> i32 {
        i + 1
    }

    let closure_annotated = |i: i32| -> i32 { i + 1 };
    let closeure_inferred = |i| i + 1;

    // 闭包表达式产生的类型就是闭包类型，不属于引用类型
    let i = 1;

    println!("function: {}", function(i));
    println!("closure_annotated: {}", closure_annotated(i));
    println!("closure_inferred: {}", closeure_inferred(i));

    // 返回类型是自动推导的。
    let one = || 1;
    println!("closure returning one: {}", one());
}

pub mod capture {
    use std::mem;

    pub fn it_work() {
        println!("++++++++++++++CLOSURE CAPTURE+++++++++++++++++");

        let color = String::from("green");

        // 这个闭包打印color，它会立即借用（通过引用&）color并将该借用和闭包本身储存到print变量中
        // color会一直保持被借用状态知道print离开作用域
        let print = || println!("color: {}", color);

        print();

        let _color_moved = color;

        let mut count = 0;
        // 这个闭包使得count的值增加，所以使用可变借用
        let mut inc = || {
            count += 1;
            println!("count: {}", count);
        };

        inc();

        // 闭包不再借用&mut count，因此可以正确地重新借用
        let _count_reborrowed = &mut count;

        // 不可复制类型
        let movable = Box::new(3);

        // mem::drop要求T类型本身，所以闭包将会捕获变量的值。这种情况下，
        // 可复制类型将会复制给闭包，从而原始值不受影响。
        // 不可复制类型必须move
        let consume = || {
            println!("movable: {:?}", movable);
            mem::drop(movable);
        };

        consume();

        let haystack = vec![1, 2, 3];

        let contains = move |needle| haystack.contains(needle);

        println!("{}", contains(&1));
        println!("{}", contains(&4));

        // println!("There're {} elements in vec", haystack.len()); // error

        // 在闭包的签名中删除move会导致闭包以不可变的方式借用haystack，因此之后haystack仍然可用，取消上面的注释不会报错
    }

    fn apply<F>(f: F)
    where
        // 闭包没有输入值和返回值
        F: FnOnce(),
    {
        f()
    }

    // 输入闭包，返回一个i32整型的函数
    fn apply_to_3<F>(f: F) -> i32
    where
        F: Fn(i32) -> i32,
    {
        f(3)
    }

    pub fn closure_in_work() {
        let greeting = "hello";

        // 不可复制的类型
        // to_owned从借用的数据创建有所有权的数据
        let mut farewell = "goodbye".to_owned();

        let diary = || {
            println!("I said {}", greeting);

            farewell.push_str("!!!");
            println!("Then I screamed {}.", farewell);
            println!("Now I can sleep. zzzzz");

            mem::drop(farewell);
        };

        apply(diary);

        let double = |x| 2 * x;
        println!("3 doubled: {}", apply_to_3(double));

        let print = || println!("{}", 7);
        apply(print);

        let closure = || println!("我是闭包");
        call_me(closure);
        call_me(function);

        println!("+++++++++++++++++闭包作为输出参数++++++++++++++");

        let fn_plain = create_fn();
        let mut fn_mut = create_fnmut();
        let fn_once = create_fnonce();

        fn_plain();
        fn_mut();
        fn_once();
    }

    fn call_me<F: Fn()>(f: F) {
        f()
    }

    fn function() {
        println!("I am function!");
    }

    fn create_fn() -> impl Fn() {
        let text = "Fn".to_owned();

        move || println!("this is a: {}", text)
    }

    fn create_fnmut() -> impl FnMut() {
        let text = "FnMut".to_owned();

        move || println!("This is a: {}", text)
    }

    fn create_fnonce() -> impl FnOnce() {
        let text = "FnOnce".to_owned();

        move || println!("This is a: {}", text)
    }
}
