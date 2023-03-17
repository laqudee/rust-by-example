#[allow(dead_code)]
struct S; // 具体类型S
#[allow(dead_code)]
struct GenericVal<T>(T);

// 此处显式地指出了类型参数
impl GenericVal<f32> {}

// 指定S类型
impl GenericVal<S> {}

// <T>必须在类型之前写出来，以使类型T代表泛型
impl<T> GenericVal<T> {}

struct Val {
    val: f64,
}

struct GenVal<T> {
    gen_val: T,
}

impl Val {
    fn value(&self) -> &f64 {
        &self.val
    }
}

impl<T> GenVal<T> {
    fn value(&self) -> &T {
        &&self.gen_val
    }
}

pub fn play() {
    let x = Val { val: 3.0 };
    let y = GenVal { gen_val: 3i32 };

    println!("{}, {}", x.value(), y.value());
}
