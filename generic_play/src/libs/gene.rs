
#[allow(dead_code)]
struct A;

#[allow(dead_code)]
struct Single(A);

#[allow(dead_code)]
struct SingleGen<T>(T);

#[allow(dead_code)]
pub fn it_work() {
    let _s = Single(A);

    let _char: SingleGen<char> = SingleGen('a');

    let _t = SingleGen(A);
    let _i32 = SingleGen(6);
    let _char = SingleGen('a');
}
