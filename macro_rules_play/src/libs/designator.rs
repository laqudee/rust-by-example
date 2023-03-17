macro_rules! create_function {
    // 此宏接受一个ident指示符表示的参数，并创建$func_name的函数
    ($func_name:ident) => {
        fn $func_name() {
            // stringify!宏把ident转换成字符串
            println!("You called {:?}()", stringify!($func_name))
        }
    };
}

create_function!(foo);

create_function!(bar);

macro_rules! print_result {
    ($expression:expr) => {
        println!("{:?} = {:?}", stringify!($expression), $expression)
    };
}

pub fn play() {
    foo();
    bar();

    print_result!(1u32 + 1);

    print_result!({
        let x = 1u32;
        x * x + 2 * x - 1
    })
}
