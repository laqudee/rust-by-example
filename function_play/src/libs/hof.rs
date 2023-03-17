fn is_odd(n: u32) -> bool {
    n % 2 == 1
}

pub fn it_work() {
    let upper = 1000;

    let mut acc = 0;

    for n in 0.. {
        let n_squared = n * n;

        if n_squared >= upper {
            break;
        } else if is_odd(n_squared) {
            acc += n_squared;
        }
    }

    println!("imperative style: {}", acc);
}

// 高阶函数写法
pub fn hof_work() {
    let upper = 1000;
    let sum_of_squared_odd_numbers: u32 = (0..)
        .map(|n| n * n)
        .take_while(|&n| n < upper)
        .filter(|&n| is_odd(n))
        .fold(0, |sum, i| sum + i);

    println!("HOF result: {}", sum_of_squared_odd_numbers);
}

// 发散函数
pub fn sum_odd_numbers(up_to: u32) -> u32 {
    let mut acc = 0;
    for i in 0..up_to {
        let addition: u32 = match i % 2 == 1 {
            true => i,
            false => continue,
        };
        acc += addition;
    }
    acc
}
