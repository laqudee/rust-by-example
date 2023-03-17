mod libs;

fn main() {
    println!("+++++++++++++++++RAII++++++++++++++++++++++");
    libs::raii::play();

    println!("+++++++++++++++++++所有权与移动++++++++++++++++++");
    libs::ownership_move::play();

    println!("+++++++++++++++借用++++++++++++++++++++");
    libs::borrow::play();
    libs::borrow::alias_play();
    libs::borrow::ref_paly();
}
