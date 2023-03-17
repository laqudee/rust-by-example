mod libs;

fn main() {
    println!("++++++++++++++thread+++++++++++++++");
    libs::thread::play();
    
    println!("++++++++++++++++Thread: Map Reduce++++++++++++++");
    libs::thread::map_reduce_play();

    println!("+++++++++++++++通道+++++++++++++++++++");
    libs::channel::play();

    println!("+++++++++++Path++++++++++++++++++++");
    libs::path::play();

    println!("++++++++++++++++++File Open++++++++++++");
    libs::file_open::play();

    println!("++++++++++++++++++File Create++++++++++++");
    libs::file_create::play();

    println!("+++++++++++++++++Read line+++++++++++++++");
    libs::file_open_lines::play();
}
