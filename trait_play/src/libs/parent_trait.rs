trait Person {
    fn name(&self) -> String;
}

trait Student: Person {
    fn university(&self) -> String;
}

trait Programmer {
    fn fav_language(&self) -> String;
}

trait CompSciStudent: Programmer + Student {
    fn git_username(&self) -> String;
}

struct CollectStudent {
    name: String,
    university: String,
    fav_language: String,
    git_username: String,
}
impl Person for CollectStudent {
    fn name(&self) -> String {
        self.name.to_string()
    }
}

impl Student for CollectStudent {
    fn university(&self) -> String {
        self.university.to_string()
    }
}

impl Programmer for CollectStudent {
    fn fav_language(&self) -> String {
        println!("{} fav language is {}", self.name, self.fav_language);
        self.fav_language.to_string()
    }
}

impl CompSciStudent for CollectStudent {
    fn git_username(&self) -> String {
        println!("{} git username is {}", self.name, self.git_username);
        self.git_username.to_string()
    }
}

fn comp_sci_student_greeting(student: &dyn CompSciStudent) -> String {
    format!(
        "My name is {} and I attend {}. My favorite language is {}. My Git username is {}",
        student.name(),
        student.university(),
        student.fav_language(),
        student.git_username()
    )
}

pub fn play() {
    let student = CollectStudent {
        name: String::from("Li Hua"),
        university: String::from("HRBNU"),
        fav_language: String::from("RUST"),
        git_username: String::from("lihua2022"),
    };

    println!("{}", comp_sci_student_greeting(&student));
}
