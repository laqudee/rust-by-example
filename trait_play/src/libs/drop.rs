struct Droppable {
    name: &'static str,
}

impl Drop for Droppable {
    fn drop(&mut self) {
        println!("> Droppable {}", self.name);
    }
}

pub fn play() {
    let _a = Droppable { name: "a" };

    {
        let _b = Droppable { name: "b" };

        {
            let _c = Droppable { name: "c" };
            let _d = Droppable { name: "d" };

            println!("Exiting block B");
        }

        println!("Just exited block B");

        println!("Exiting block A");
    }

    println!("Just exited block A");

    // 变量可以手动使用 `drop` 函数来销毁。
    drop(_a);
    // 试一试 ^ 将此行注释掉。

    println!("end of the main function");
}
