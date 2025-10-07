use common_serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Person {
    name: String,
    age: i32,
    is_active: bool,
    next_person: Option<Box<Person>>,
}

fn main() {
    let first_person = Person {
        name: "Alice".to_string(),
        age: 30,
        is_active: true,
        next_person: None,
    };

    let second_person = Person {
        name: "Alice".to_string(),
        age: 30,
        is_active: true,
        next_person: Some(Box::new(first_person)),
    };

    let mut buffer: Vec<u8> = Vec::new();
    second_person.serialize(&mut buffer).unwrap();
    println!("После сериализации: {:?}", buffer);

    let mut deserializer = buffer.as_slice();
    let deserialized_person = Person::deserialize(&mut deserializer).unwrap();
    println!("После десериализации: {:?}", deserialized_person);

    assert_eq!(second_person, deserialized_person);
}
