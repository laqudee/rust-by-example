use std::path::Path;

pub fn play() {
    let path = Path::new(".");

    let display = path.display();
    println!("path dispath: {}", display);

    let new_path = path.join("a").join("b");

    match new_path.to_str() {
        None => panic!("new path is not a valid UTF-8 sequence"),
        Some(s) => println!("new path is {}", s),
    }
}
