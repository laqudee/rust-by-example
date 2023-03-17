use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub fn play() {
    let path = Path::new("hello.txt");
    let display = path.display();

    // 以只读方式打开路径，返回io::Result<File>
    let mut file = match File::open(&path) {
        Err(why) => panic!("could't open {}: {:?}", display, why),
        Ok(file) => file,
    };

    // 读文件内容到一个字符串，返回io::Result<usize>
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {:?}", display, why),
        Ok(_) => print!("{} contains:\n{}", display, s),
    }
}
