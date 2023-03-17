pub struct Point {
    x: f64,
    y: f64,
}

impl Point {
    // 静态方法static method
    // static method不能被实例调用
    // 一般作为构造器使用
    pub fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }

    // 构造函数
    pub fn new(x: f64, y: f64) -> Point {
        Point { x, y }
    }
}

pub struct Rectangle {
    pub p1: Point,
    pub p2: Point,
}

impl Rectangle {
    // 实例方法，可以被实例访问
    pub fn area(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        (x1 - x2).abs() * (y1 - y2).abs()
    }

    pub fn perimeter(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
    }

    // 这个方法要求调用者是可变的
    pub fn translate(&mut self, x: f64, y: f64) {
        self.p1.x += x;
        self.p2.x += x;

        self.p1.y += y;
        self.p2.y += y;
    }
}

pub struct Pair(Box<i32>, Box<i32>);

impl Pair {
    pub fn new(box1: Box<i32>, box2: Box<i32>) -> Pair {
        Pair(box1, box2)
    }
    pub fn destroy(self) {
        let Pair(first, second) = self;

        println!("Destroying Pair ({}, {})", first, second);

        // first,second 离开作用域后释放
    }
}
