use serde::{Deserialize, Serialize};

fn main() {
    let me = Person {
        name: String::from("Luke"),
        age: 33,
        hobbies: Vec::from([
            Hobby {
                name: String::from("Woodworking"),
                years: 20,
            },
            Hobby {
                name: String::from("Motorcycling"),
                years: 10,
            },
        ]),
    };

    let serialized_user = serde_json::to_string(&me).unwrap();

    println!("{:?}", serialized_user);
}

#[derive(Serialize, Deserialize, Debug)]
struct Person {
    name: String,
    age: i32,
    hobbies: Vec<Hobby>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Hobby {
    name: String,
    years: i32,
}
