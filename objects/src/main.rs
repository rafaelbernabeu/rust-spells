mod enums;
mod person;

use crate::person::Person;
use std::string::String;
use std::io::Result;

fn main() {
    let person: Person = Person::builder(String::from("Rafous"), 29, 'M');

    println!("{:#?}", person);
}
