fn main() {
    let x = 5u32;

    let y = {
        let x_s = x * x;
        let x_cube = x_s * x;

        x_cube + x_s + x
    };

    let z = {
        2 * x;
    };

    println!("x is {:?}", x);
    println!("y is {:?}", y);
    println!("z is {:?}", z);
}
