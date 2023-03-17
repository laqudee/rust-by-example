use std::fmt::Debug;

trait HasArea {
    fn area(&self) -> f64;
}

impl HasArea for Rectangle {
    fn area(&self) -> f64 {
        self.length * self.height
    }
}

#[derive(Debug)]
struct Rectangle {
    length: f64,
    height: f64,
}

#[allow(dead_code)]
struct Triangle {
    length: f64,
    height: f64,
}

fn print_debug<T: Debug>(t: &T) {
    println!("{:?}", t);
}

fn area<T: HasArea>(t: &T) -> f64 {
    t.area()
}

pub fn play() {
    let rectangle = Rectangle {
        length: 3.0,
        height: 4.0,
    };

    let _triangle = Triangle {
        length: 3.0,
        height: 4.0,
    };

    print_debug(&rectangle);

    println!("Area: {}", area(&rectangle));

    // error 为实现Debug 或 HasArea
    // print_debug(&_triangle);
    // println!("Area: {}", area(&_triangle));
}

struct Cardinal;
struct BlueJay;
struct Turkey;

trait Red {}
trait Blue {}

impl Red for Cardinal {}
impl Blue for BlueJay {}

// 这些函数只对实现了相应的trait的类型有效
// 事实上这些trait内部是空的，但没有关系
fn red<T: Red>(_: &T) -> &'static str {
    "red"
}
fn blue<T: Blue>(_: &T) -> &'static str {
    "blue"
}

pub fn null_bound_play() {
    let cardinal = Cardinal;
    let blue_jay = BlueJay;
    let _turkey = Turkey;

    println!("A cardinal is {}", red(&cardinal));
    println!("A blue jay is {}", blue(&blue_jay));
    // error
    // println!("A turkey is {}", red(&_turkey));
}
