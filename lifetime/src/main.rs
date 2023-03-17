mod libs;

fn main() {
    println!("++++++++++显式标注+++++++++");
    libs::explicit::play();

    println!("++++++++++函数++++++++++");
    libs::function::play();

    println!("++++++++++方法++++++++++");
    libs::methods::play();

    println!("++++++++++结构体++++++++++");
    libs::structure::play();

    println!("+++++++++++trait++++++++++");
    libs::r#trait::play();

    println!("+++++++++++约束++++++++++");
    libs::bound::play();

    println!("+++++++++++强制转换++++++++++");
    libs::cast_type::play();

    println!("++++++++++++static+++++++++");
    libs::r#static::play();

    println!("+++++++++++省略++++++++++");
    libs::elision::play();
}
