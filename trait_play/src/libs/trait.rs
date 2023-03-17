struct Sheep {
    naked: bool,
    name: &'static str,
}

trait Animal {
    // 静态方法签名；Self表示实现者类型implementor type
    fn new(name: &'static str) -> Self;

    // 实例方法签名；
    fn name(&self) -> &'static str;
    fn noise(&self) -> &'static str;

    // trait 可以默认的方法定义
    fn talk(&self) {
        println!("{} says {}", self.name(), self.noise());
    }
}

impl Sheep {
    fn is_naked(&self) -> bool {
        self.naked
    }

    fn shear(&mut self) {
        if self.is_naked() {
            // 实现者可以使用它的trait方法
            println!("{} is already naked ...", self.name());
        } else {
            println!("{} gets a haircut!", self.name);

            self.naked = true;
        }
    }
}

impl Animal for Sheep {
    fn new(name: &'static str) -> Sheep {
        Sheep {
            naked: false,
            name,
        }
    }

    fn name(&self) -> &'static str {
        self.name
    }

    fn noise(&self) -> &'static str {
        if self.is_naked() {
            "baaaaah?"
        } else {
            "baaaaah!"
        }
    }

    // 默认trait方法可以重载
    fn talk(&self) {
        println!("{} pauses briefly... {}", self.name, self.noise());
    }
}

pub fn play() {
    let mut dolly: Sheep = Animal::new("Dolly");
    // let mut dolly = Sheep::new("Dolly");

    dolly.talk();
    dolly.shear();
    dolly.talk();
}
