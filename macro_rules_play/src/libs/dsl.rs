macro_rules! calculate {
    (eval $e:expr) => {{
        {
            let val: usize = $e; // 强制类型转为整数
            println!("{}  = {}", stringify!($e), val);
        }
    }};

    // 递归地拆解多重的eval
    (eval $e:expr, $(eval $es:expr),+) => {{
        calculate! {eval $e}
        calculate! { $(eval $es),+ }
    }}
}

pub fn play() {
    calculate! {
        eval 1 + 1
    }

    calculate! {
        eval (1 + 2) * (3 / 4)
    }

    // 可变参数
    calculate! {
        eval 1 + 2,
        eval 3 + 4,
        eval (2 * 3) + 1
    }
}
