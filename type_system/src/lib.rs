pub mod type_casting {
    pub fn play() {
        let decimal = 65.4321_f32;

        // 错误！不提供隐式转换
        // let integer: u8 = decimal;

        // 显式转换
        let integer = decimal as u8;
        let character = integer as char;

        println!("Casting: {} -> {} -> {}", decimal, integer, character);
    }

    pub fn literal_play() {
        let x = 1u8;
        let y = 2u32;
        let z = 3f32;

        let i = 1;
        let f = 1.6;

        // size_of_val()返回一个变量所占的字节数
        println!("size of x in bytes: {}", std::mem::size_of_val(&x));
        println!("size of `y` in bytes: {}", std::mem::size_of_val(&y));
        println!("size of `z` in bytes: {}", std::mem::size_of_val(&z));
        println!("size of `i` in bytes: {}", std::mem::size_of_val(&i));
        println!("size of `f` in bytes: {}", std::mem::size_of_val(&f));
    }
}

pub mod type_what {
    pub fn play() {
        let elem = 5u8;

        let mut vec = Vec::new();

        vec.push(elem);

        println!("{:?}", vec);
    }

    type NanoSecond = u64;
    type Inch = u64;

    #[allow(non_camel_case_types)]
    type u64_t = u64;

    pub fn alias_play() {
        let nanoseconds: NanoSecond = 5 as u64_t;
        let inches: Inch = 2 as u64_t;

        println!(
            "{} nanoseconds + {} inches = {} unit?",
            nanoseconds,
            inches,
            nanoseconds + inches
        );
    }
}
