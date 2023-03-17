pub mod if_else {
    pub fn play() {
        let n = 5;
        let big_n = if n < 10 && n > -10 {
            println!(", and is a small number, increase ten-fold");
            10 * n
        } else {
            println!(", and is a  bug number the number");
            n / 2
        };

        println!("{} -> {}", n, big_n);
    }
}

pub mod loop_play {
    #![allow(unreachable_code)]
    pub fn play() {
        let mut count = 0u32;

        println!("Let's count until inifinity");

        loop {
            count += 1;

            if count == 3 {
                println!("three");
                continue;
            }

            println!("{}", count);

            if count == 5 {
                println!("OK, that is enough");

                break;
            }
        }
    }

    pub fn label_loop_play() {
        'outer: loop {
            println!("Enter the outer loop");

            'inner: loop {
                println!("Entered the inner loop");

                // 退出内层循环
                // break;

                // 退出外层循环
                break 'outer;
            }

            println!("This point will never be reached");
        }

        println!("exited the outer loop");
    }

    pub fn loop_break_value() {
        let mut counter = 0;

        let result = loop {
            counter += 1;

            if counter == 10 {
                break counter * 2;
            }
        };

        println!("break value of loop_break_value is {}", result);
    }
}

pub mod while_play {
    pub fn play() {
        let mut n = 1;

        while n < 101 {
            if n % 15 == 0 {
                println!("fizzbuzz");
            } else if n % 3 == 0 {
                println!("fizz");
            } else if n % 5 == 0 {
                println!("buzz");
            } else {
                println!("{}", n);
            }

            n += 1;
        }
    }
}

pub mod for_play {
    // a..b 和 a..=b-1都可以
    pub fn play() {
        // for n in 1..101 {
        for n in 1..=100 {
            if n % 15 == 0 {
                println!("fizzbuzz");
            } else if n % 3 == 0 {
                println!("fizz");
            } else if n % 5 == 0 {
                println!("buzz");
            } else {
                println!("{}", n);
            }
        }
    }

    pub fn for_iter_play() {
        // iter，在每次迭代中借用集合中的一个元素。这样集合本身不会被改变，循环之后仍可以用
        let names = vec!["Bob", "Frank", "Ferris"];

        for name in names.iter() {
            match name {
                &"Ferris" => println!("There is a rustacean among us!"),
                _ => println!("Hello {}", name),
            }
        }
    }

    pub fn for_intoiter_play() {
        // into_iter，会消耗集合。在每次迭代中，集合中的数据本身会被提供，一旦消耗了，之后就无法在使用了
        let names = vec!["Bob", "Frank", "Ferris"];

        for name in names.into_iter() {
            match name {
                "Ferris" => println!("There is a rustacean among us!"),
                _ => println!("Hello {}", name),
            }
        }
        // error
        // println!("{}", names[0]);
    }

    pub fn for_itermut_play() {
        // iter_mut，可变地借用集合中的每个元素，从而允许集合被就地修改
        let mut names = vec!["Bob", "Frank", "Ferris"];

        for name in names.iter_mut() {
            *name = match name {
                &mut "Ferris" => "There is a rustacean among us!",
                _ => "Hello",
            }
        }
        println!("names: {:?}", names);
    }
}

pub mod match_play {
    pub fn play() {
        let number = 13;

        match number {
            1 => println!("One!"),
            2 | 3 | 5 | 7 | 11 => println!("This is a prime"),
            13..=19 => println!("A teen"),
            _ => println!("Ain't special"),
        }

        let boolean = true;
        // match 也是一个表达式
        let binary = match boolean {
            // match 分支必须覆盖所有可能的值
            false => 0,
            true => 1,
            // 试一试 ^ 将其中一条分支注释掉
        };

        println!("{} -> {}", boolean, binary);
    }

    pub fn tuple_play() {
        // 元组可以在match中解构
        let triple = (0, -2, 3);

        println!("Tell me about {:?}", triple);
        match triple {
            (0, y, z) => println!("First is 0, y is {:?}, z is {:?}", y, z),
            (1, ..) => println!("First is 1, and the rest doesn't matter"),
            _ => println!("It doesn't matter what they are"),
        }
    }

    pub fn enum_play() {
        // 解构枚举
        #[allow(dead_code)]
        enum Color {
            // 这三个取值仅由它们的名字而非类型来指定
            Red,
            Blue,
            Green,
            // 这些则把u32元组赋予不同的名字，以色彩模型命名
            RGB(u32, u32, u32),
            HSV(u32, u32, u32),
            HSL(u32, u32, u32),
            CMY(u32, u32, u32),
            CMYK(u32, u32, u32, u32),
        }

        let color = Color::RGB(122, 17, 40);

        match color {
            Color::Red => println!("The color is Red!"),
            Color::Blue => println!("The color is Blue!"),
            Color::Green => println!("The color is Green!"),
            Color::RGB(r, g, b) => println!("Red: {}, green: {}, and blue: {}!", r, g, b),
            Color::HSV(h, s, v) => println!("Hue: {}, saturation: {}, value: {}!", h, s, v),
            Color::HSL(h, s, l) => println!("Hue: {}, saturation: {}, lightness: {}!", h, s, l),
            Color::CMY(c, m, y) => println!("Cyan: {}, magenta: {}, yellow: {}!", c, m, y),
            Color::CMYK(c, m, y, k) => println!(
                "Cyan: {}, magenta: {}, yellow: {}, key (black): {}!",
                c, m, y, k
            ),
            // 不需要其它分支，因为所有的情形都已覆盖
        }
    }

    pub fn ref_play() {
        // 获取一个i32类型的引用，&表示取引用
        let reference = &4;

        match reference {
            &val => println!("Got a value via destructuring: {:?}", val),
        }

        // 如果不想使用&，需要在匹配前解引用
        match *reference {
            val => println!("Got a value via dereferencing: {:?}", val),
        }

        let _not_a_ref = 3;

        let ref _is_a_ref = 3;

        let value = 5;
        let mut mut_value = 6;

        // 使用ref关键字创建引用
        match value {
            ref r => println!("Got a reference to a value: {:?}", r),
        }

        match mut_value {
            ref mut m => {
                *m += 10;
                println!("We added 10, mut_value: {:?}", m)
            }
        }
    }

    pub fn struct_play() {
        struct Foo {
            x: (u32, u32),
            y: u32,
        }

        let foo = Foo { x: (1, 2), y: 3 };
        let Foo { x: (a, b), y } = foo;
        println!("a = {}, b = {},  y = {} ", a, b, y);

        let Foo { y: i, x: j } = foo;
        println!("i = {:?}, j = {:?}", i, j);

        // 也可以忽略某些变量
        let Foo { y, .. } = foo;
        println!("y = {}", y);
    }

    pub fn guard_play() {
        let pair = (2, -2);

        println!("Tell me about {:?}", pair);

        match pair {
            (x, y) if x == y => println!("These are teins"),
            (x, y) if x + y == 0 => println!("等于0"),
            (x, _) if x % 2 == 1 => println!("x可以被整除"),
            _ => println!("No correlation..."),
        }
    }

    pub fn bingding_play() {
        match age() {
            0 => println!("I haven't celebrated my first birthday yet"),
            n @ 1..=12 => println!("I'm a child of age {:?}", n),
            n @ 13..=19 => println!("I'm a teen of age {:?}", n),
            // 不符合上面的范围。返回结果。
            n => println!("I'm an old person of age {:?}", n),
        }

        match some_number() {
            // 得到 `Some` 可变类型，如果它的值（绑定到 `n` 上）等于 42，则匹配。
            Some(n @ 42) => println!("The Answer: {}!", n),
            // 匹配任意其他数字。
            Some(n) => println!("Not interesting... {}", n),
            // 匹配任意其他值（`None` 可变类型）。
            _ => (),
        }
    }

    fn age() -> u32 {
        15
    }

    fn some_number() -> Option<u32> {
        Some(42)
    }
}

pub mod if_let_play {
    pub fn play() {
        let number = Some(7);
        let letter: Option<i32> = None;
        let emoticon: Option<i32> = None;

        // if let 读作：若let将number解构成Some(i)则执行语句块{}
        if let Some(i) = number {
            println!("Matched {:?}", i);
        }

        // 若要指明失败情形，使用else
        if let Some(i) = letter {
            println!("Matched {:?}", i);
        } else {
            println!("Didn't match a number, Let's go with a letter");
        }

        // 提供另一种失败情况下的条件。
        let i_like_letters = false;

        if let Some(i) = emoticon {
            println!("Matched {:?}!", i);
        // 解构失败。使用 `else if` 来判断是否满足上面提供的条件。
        } else if i_like_letters {
            println!("Didn't match a number. Let's go with a letter!");
        } else {
            // 条件的值为 false。于是以下是默认的分支：
            println!("I don't like letters. Let's go with an emoticon :)!");
        };
    }
}

pub mod while_let_play {
    pub fn play() {
        let mut optional = Some(0);

        while let Some(i) = optional {
            if i > 9 {
                println!("Greater than 9, quit");
                optional = None;
            } else {
                println!("i is {:?}, try again", i);
                optional = Some(i + 1);
            }
        }
    }
}
