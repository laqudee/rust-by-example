// 可以比较的元组结构体
#[derive(PartialEq, PartialOrd)]
struct Centimeters(f64);

// 可以打印的元组结构体
#[derive(Debug)]
struct Inches(i32);

impl Inches {
    fn to_centimeters(&self) -> Centimeters {
        let &Inches(inches) = self;

        Centimeters(inches as f64 * 2.54)
    }
}

// 不带附加属性的元组结构体
struct Second(i32);

pub fn play() {
    let _one_second = Second(1);

    // 没有实现Debug trait 所以无法打印
    // println!("one second: {:?}", _one_second);

    // 报错：`Seconds`不能比较；它没有实现 `PartialEq` trait
    // let _this_is_true = (_one_second == _one_second);

    let foot = Inches(12);
    println!("foot is {:?}", foot);

    let meter = Centimeters(100.0);
    let cmp = 
        if foot.to_centimeters() < meter {
            "smaller"
        } else {
            "bigger"
        };
    println!("one foot is {} than one meter", cmp);
}