mod libs;

fn main() {
    libs::one::play();

    println!("+++++++++++++++++++指示符++++++++++++++++++");
    libs::designator::play();

    println!("+++++++++++++++++++重载++++++++++++++++++++");
    libs::overloading::play();

    println!("+++++++++++++++++++重复++++++++++++++++++++");
    libs::repeat::play();

    println!("+++++++++++++++++DRY+++++++++++++++++++");
    libs::dry::play();

    println!("++++++++++++++++++DSL与可变参数++++++++++++++++++++");
    libs::dsl::play();
}
