mod libs;

use libs::methods::{Pair, Point, Rectangle};

use libs::closure;

use libs::iterator;

fn main() {
    libs::function::it_work();
    libs::function::fizzbuzz_to(1);

    println!("++++++++++++++++++Method+++++++++++++++");

    let rectangle = Rectangle {
        p1: Point::origin(),
        p2: Point::new(3.0, 4.0),
    };

    println!("Rectangle perimeter: {}", rectangle.perimeter());
    println!("Rectange area: {}", rectangle.area());

    let mut square = Rectangle {
        p1: Point::origin(),
        p2: Point::new(1.0, 1.0),
    };

    square.translate(1.0, 1.0);

    let pair = Pair::new(Box::new(1), Box::new(2));

    pair.destroy();

    println!("++++++++++++++++++++++CLOSURE++++++++++++++++++++++++");

    closure::it_work();

    closure::capture::it_work();

    closure::capture::closure_in_work();

    println!("+++++++++++++++++++示例+++++++++++++++++");

    iterator::it_work();

    println!("++++++++++++++++++++高级函数+++++++++++++");

    libs::hof::it_work();
    libs::hof::hof_work();

    println!("+++++++++++++++++++发散函数++++++++++++++");

    println!("Sum of odd numbers up to 9 (excluding): {}", libs::hof::sum_odd_numbers(9));
}
