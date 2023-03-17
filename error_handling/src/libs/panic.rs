fn give_princess(gift: &str) {
    if gift == "snake" {
        panic!("AAAAAaaaa!!!");
    }

    println!("I love {}s!!!!!", gift);
}

pub fn play() {
    give_princess("teddy bear");
    give_princess("snake");
}
