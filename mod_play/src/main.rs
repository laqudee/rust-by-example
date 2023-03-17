use mod_play::deep::other_function;
use mod_play::mode;
use mod_play::my_mod;
use mod_play::my_mod_test;

fn function() {
    println!("called function()");
}

mod my {
    // 一个公有的结构体，带有一个公有的字段
    pub struct OpenBox<T> {
        pub contents: T,
    }

    #[allow(dead_code)]
    #[derive(Debug)]
    pub struct ClosedBox<T> {
        contents: T,
    }

    impl<T> ClosedBox<T> {
        pub fn new(contents: T) -> ClosedBox<T> {
            ClosedBox { contents }
        }
    }
}

fn main() {
    function();

    other_function();

    println!("Entering block");
    {
        // 这和use deep::nested2::function as function 等价
        use mod_play::deep::nested2::function;
        function();
        println!("Leaving block");
    }

    my_mod::indirect_access();
    my_mod::nested::function();
    my_mod::call_public_function_in_my_mod();
    my_mod_test();

    let open_box = my::OpenBox {
        contents: "public information ",
    };

    println!("The open box contains: {}", open_box.contents);

    // 带有私有字段的公有结构体不能使用字段名来构造
    // 只能使用公有的构造器来创建
    let closed_box = my::ClosedBox::new("classified information");
    println!("The closed box : {:?}", closed_box);

    mode::my::indirect_call();
}
