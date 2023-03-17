use std::mem::drop;

#[derive(Debug, Clone, Copy)]
struct Nil;

#[derive(Debug, Clone)]
struct Pair(Box<i32>, Box<i32>);

pub fn play() {
    let nil = Nil;
    let copied_nil = nil;

    // 两个 `Nil` 都可以独立使用
    println!("original: {:?}", nil);
    println!("copy: {:?}", copied_nil);

    // 实例化Pair
    let pair = Pair(Box::new(1), Box::new(2));
    println!("original: {:?}", pair);

    let moved_pair = pair;
    println!("copy: {:?}", moved_pair);

    // 报错！`pair` 已失去了它的资源。
    //println!("original: {:?}", pair);

    let cloned_pair = moved_pair.clone();
    // 使用 std::mem::drop 来销毁原始的 pair。
    drop(moved_pair);

    // 报错！`moved_pair` 已被销毁。
    //println!("copy: {:?}", moved_pair);

    // 由 .clone() 得来的结果仍然可用！
    println!("clone: {:?}", cloned_pair);
}
