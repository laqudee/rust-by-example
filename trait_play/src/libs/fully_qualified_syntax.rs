trait UsernameWidget {
    fn get(&self) -> String;
}

trait AgeWidget {
    fn get(&self) -> u8;
}

struct Form {
    username: String,
    age: u8,
}

impl UsernameWidget for Form {
    fn get(&self) -> String {
        self.username.clone()
    }
}

impl AgeWidget for Form {
    fn get(&self) -> u8 {
        self.age
    }
}

pub fn play() {
    let form = Form {
        username: "rustacean".to_owned(),
        age: 20,
    };

    let username = <Form as UsernameWidget>::get(&form);
    let age = <Form as AgeWidget>::get(&form);

    println!("username: {}, age: {}", username, age);
}