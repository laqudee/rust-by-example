pub mod type_from_info {
    use std::convert::From;

    #[derive(Debug)]
    struct Number {
        value: i32,
    }

    impl From<i32> for Number {
        fn from(item: i32) -> Self {
            Number { value: item }
        }
    }

    pub fn play() {
        let num = Number::from(30);
        println!("My number is {:?}", num);

        let int = 5;
        let int_into: Number = int.into();
        println!("My int_into is {:?}", int_into);
    }
}

pub mod type_tryfrom_tryinfo {
    use std::convert::TryFrom;
    use std::convert::TryInto;

    #[derive(Debug, PartialEq)]
    struct EvenNumber(i32);

    impl TryFrom<i32> for EvenNumber {
        type Error = ();

        fn try_from(value: i32) -> Result<Self, Self::Error> {
            if value % 2 == 0 {
                Ok(EvenNumber(value))
            } else {
                Err(())
            }
        }
    }

    pub fn play() {
        // TryFrom
        assert_eq!(EvenNumber::try_from(8), Ok(EvenNumber(8)));
        assert_eq!(EvenNumber::try_from(5), Err(()));

        // TryInto

        let result: Result<EvenNumber, ()> = 8i32.try_into();
        assert_eq!(result, Ok(EvenNumber(8)));
        let result: Result<EvenNumber, ()> = 5i32.try_into();
        assert_eq!(result, Err(()));
    }
}

pub mod type_tostring_fromstr {
    use std::fmt;
    use std::string::ToString;

    struct Circle {
        radius: i32,
    }

    impl fmt::Display for Circle {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Circle of radius {}", self.radius)
        }
    }

    pub fn play() {
        let circle = Circle { radius: 32 };
        println!("{}", circle.to_string());

        let rect = Rect { x: 10, y: 20 };
        println!("{}", rect.to_string());

        let parsed: i32 = "5".parse().unwrap();
        let turbo_parse = "10".parse::<i32>().unwrap();

        let sum = parsed + turbo_parse;
        println!("Sum: {:?}", sum);
    }

    struct Rect {
        x: u32,
        y: u32,
    }

    impl ToString for Rect {
        fn to_string(&self) -> String {
            format!("Rect of x {:?}, y {:?}", self.x, self.y)
        }
    }
}
