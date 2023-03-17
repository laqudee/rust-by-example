pub fn let_play() {
    let an_integer = 1u32;
    let a_boolean = true;

    let unit = ();

    let copied_integer = an_integer;

    println!("An integer: {:?}", copied_integer);
    println!("A boolean: {:?}", a_boolean);
    println!("Meet the unit value: {:?}", unit);

    // 编译器会对未使用的变量绑定产生警告；可以给变量名加上下划线前缀来消除警告。
    let _unused_variable = 3u32;

    let _noisy_unused_variable = 2u32;
}

pub fn mut_play() {
    let mut abs = 1;

    abs += 3;

    println!("{}", abs)
}

pub fn scope_play() {
    let long_lived_binding = 1;

    {
        let short_lived_binding = 2;

        println!("inner short: {}", short_lived_binding);

        // 此绑定*遮蔽*了外面的绑定
        let long_lived_binding = 5_f32;

        println!("inner long: {}", long_lived_binding);
    }

    // error
    // println!("outer short: {}", short_live_binding);

    println!("outer long: {}", long_lived_binding);

    // 此绑定同样*遮蔽*了前面的绑定
    let long_lived_binding = 'a';

    println!("outer long: {}", long_lived_binding);
}

pub fn declare_play() {
    let a_binding;

    {
        let x = 2;

        a_binding = x * x;
    }

    println!("a binding: {}", a_binding);
}

pub fn freeze_play() {
    let mut _mutable_integer = 7i32;

    {
        // 被不可变遮蔽
        let _mutable_integer = _mutable_integer;

        // error,
        // _mutable_integer = 80;
    }

    _mutable_integer = 40;

    println!("_mutable_integer: {}", _mutable_integer);
}
