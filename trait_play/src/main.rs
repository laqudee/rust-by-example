mod libs;

fn main() {
    libs::r#trait::play();
    
    println!("+++++++++++++++派生+++++++++++++++++++");
    libs::derive::play();

    println!("++++++++++++++++dyn+++++++++++++++++");
    libs::r#dyn::play();

    println!("++++++++++++++++运算符重载+++++++++++++++++");
    libs::ops_add::play();

    println!("+++++++++++++++Drop+++++++++++++++++");
    libs::drop::play();

    println!("+++++++++++Ierator++++++++++++++++++");
    libs::iterator::play();

    println!("++++++++++++++impl Trait++++++++++++++++");
    libs::impl_trait::play();

    println!("++++++++++++++Clone+++++++++++++++");
    libs::clone::play();

    println!("++++++++ parent trait+++++++++++++++++");
    libs::parent_trait::play();

    println!("++++++fully qualified synatx++++++++++++++");
    libs::fully_qualified_syntax::play();
}
