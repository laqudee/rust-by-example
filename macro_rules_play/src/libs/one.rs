macro_rules! say_hello {
    // ()表示此宏不接受任何参数
    () => {
        // 此宏将会展开这个代码块里面的内容
        println!("Hello")
    };
}

pub fn play() {
    say_hello!()
}