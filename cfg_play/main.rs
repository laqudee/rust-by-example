#[cfg(some_condition)]
fn conditional_function() {
    println!("自定义条件编译");
}

fn main() {
    conditional_function()
}