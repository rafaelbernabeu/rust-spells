use crate::enums::SEX;

#[derive(Debug)]
pub struct Person {
    name: String,
    age: u8,
    sex: SEX
}

impl Person {
    pub fn builder(name: String, age: u8, sex: char) -> Person {
        Person {
            name,
            age,
            sex: match sex {
                'M' => SEX::MALE,
                'F' => SEX::FEMALE,
                _ => SEX::NONE
            }
        }
    }
}

