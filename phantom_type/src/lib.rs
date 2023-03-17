use std::marker::PhantomData;
use std::ops::Add;

// 这个虚元组结构体对A是泛型的，并且带有隐藏参数B
#[derive(PartialEq)] // 允许这种类型进行相等测试equality test
struct PhantomTuple<A, B>(A, PhantomData<B>);

#[derive(PartialEq)]
struct PhantomStruct<A, B> {
    first: A,
    phantom: PhantomData<B>,
}

// 创建空枚举类型来表示单位
#[derive(Debug, Clone, Copy)]
enum Inch {}
#[derive(Debug, Clone, Copy)]
enum Mm {}

/// `length` 是一个带有虚类型参数`Unit`的类型
/// f64 已经实现了 Clone和Copy trait
#[derive(Debug, Clone, Copy)]
struct Length<Unit>(f64, PhantomData<Unit>);

/// Add trait 定义了+运算符的行为
impl<Unit> Add for Length<Unit> {
    type Output = Length<Unit>;

    fn add(self, rhs: Length<Unit>) -> Length<Unit> {
        Length(self.0 + rhs.0, PhantomData)
    }
}

// 对泛型A分配存储空间，对B不会，因此B不能参与运算

pub fn play() {
    let _tuple1: PhantomTuple<char, f32> = PhantomTuple('Q', PhantomData);
    let _tuple2: PhantomTuple<char, f64> = PhantomTuple('Q', PhantomData);

    let _struct1: PhantomStruct<char, f32> = PhantomStruct {
        first: 'Q',
        phantom: PhantomData,
    };
    let _struct2: PhantomStruct<char, f64> = PhantomStruct {
        first: 'Q',
        phantom: PhantomData,
    };

    // 编译期错误！类型不匹配，所以这些值不能够比较：
    // println!("_tuple1 == _tuple2 yields: {}", _tuple1 == _tuple2);
    // println!("_struct1 == _struct2 yields: {}", _struct1 == _struct2);

    println!("++++++++++++++单位说明，单位检查++++++++++++++++");

    let one_foot: Length<Inch> = Length(12.0, PhantomData);
    let one_meter: Length<Mm> = Length(1000.0, PhantomData);

    let two_feet = one_foot + one_foot;
    let two_meters = one_meter + one_meter;

    println!("one foot + one foot = {:?} in", two_feet.0);
    println!("one meter + one meter = {:?} mm", two_meters.0)

    // 无意义的运算当然会失败：
    // 编译期错误：类型不匹配。
    // let one_feter = one_foot + one_meter;
}
