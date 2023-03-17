#[derive(Debug)]
#[allow(dead_code)]
struct Borrowed<'a> {
    x: &'a i32,
}

// 给impl标注生命周期
impl <'a> Default for Borrowed<'a> {
    fn default() -> Self {
        Self { x: &10 }
    }
}

pub fn play() {
    let b: Borrowed = Default::default();
    println!("b is {:?}", b);
}