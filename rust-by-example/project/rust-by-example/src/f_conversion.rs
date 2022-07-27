pub fn main() {
    from_and_into();
    try_from_and_try_into();
    to_and_from_strings();
}

use std::convert::From;

#[derive(Debug)]
struct MyNumber {
    value: i32,
}

impl From<i32> for MyNumber {
    fn from(item: i32) -> Self {
        MyNumber { value: item }
    }
}

fn from_and_into() {
    let num_from = MyNumber::from(30);
    println!("num_from is {:?}", num_from);
    // num_from is MyNumber { value: 30 }

    let int = 5;
    let num_to: MyNumber = int.into();
    println!("num_to is {:?}", num_to);
    // num_to is MyNumber { value: 5 }
}

use std::convert::TryFrom;
use std::convert::TryInto;

#[derive(Debug)]
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

impl PartialEq for EvenNumber {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

fn try_from_and_try_into() {
    assert_eq!(
        EvenNumber::try_from(8),
        Ok(EvenNumber(8))
    );
    assert_eq!(
        EvenNumber::try_from(5),
        Err(())
    );

    let result: Result<EvenNumber, ()> = 8i32.try_into();
    assert_eq!(
        result,
        Ok(EvenNumber(8))
    );
    let result: Result<EvenNumber, ()> = 5i32.try_into();
    assert_eq!(
        result,
        Err(())
    );
}

use std::fmt;
use std::fmt::Formatter;

struct Circle {
    radius: i32,
}

impl fmt::Display for Circle {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Circle of radius {}", self.radius)
    }
}

fn to_and_from_strings() {
    let circle = Circle { radius: 6 };
    println!("{}", circle.to_string());
    // Circle of radius 6

    let parsed: i32 = "5".parse().unwrap();
    let turbo_parsed = "10".parse::<i32>().unwrap();
    let sum = parsed + turbo_parsed;
    println!("Sum: {:?}", sum);
    // Sum: 15
}
