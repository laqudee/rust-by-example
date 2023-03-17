///显式标注

fn print_refs<'a, 'b>(x: &'a i32, y: &'b i32) {
    println!("x is {} and y is {}", x, y);
}

fn failed_borrow<'a>() {
    let _x = 12;

    // error y is live not enough
    // let y: &'a i32 = &_x;
}

pub fn play() {
    let (four, nine) = (4, 9);

    print_refs(&four, &nine);

    failed_borrow();
    // failed_borrow未包含引用，因此不要求'a长于函数的生命周期
    // 但 `'a` 寿命确实更长。因为该生命周期从未被约束，所以默认为 `'static`
}