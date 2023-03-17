use std::sync::Arc;
use std::thread;

pub fn play() {
    // 这个变量声明用来指定其值的地方
    let apple = Arc::new("the same apple");

    for _ in 0..10 {
        // 这里没有数值说明，因为他是一个指向内存堆中引用的指针
        let apple = Arc::clone(&apple);

        thread::spawn(move || {
            println!("{:?}", apple);
        });
    }
}
