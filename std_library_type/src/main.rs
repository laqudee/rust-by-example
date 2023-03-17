use std_library_type::hash_map_play;

mod libs;

fn main() {
    println!("++++++++++++++++++++散列表+++++++++++++");
    hash_map_play();

    println!("+++++++++++++++++更改或自定义关键字类型++++++++++++++");
    libs::custom_hash_map::play();

    println!("+++++++++++++++++++HashSet++++++++++++");
    libs::hash_set::play();

    println!("+++++++++++++++++++Rc++++++++++++++++++");
    libs::rc::play();

    println!("++++++++++++++++Arc++++++++++++++++++");
    libs::arc::play();
}
