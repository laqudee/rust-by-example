#[test]
fn it_work() {
    let closure = |x| x + 1;
    assert_eq!(2, closure(1));
}