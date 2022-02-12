use serde::{Deserialize, Serialize};

fn main() {
    let x: i32 = 1;
    let mut y: i32 = 2;
    println!("{} {}", x, y);
    y = 3;
    println!("{:?} {}", x, y);

    let four_ints: [i32; 4] = [1, 2, 3, 4];

    println!("{:?}", four_ints);

    println!("Hello world");

    let me = Person {
        name: String::from("Luke"),
        age: 33,
    };

    me.greet();

    println!("{:?} {:?}", me.name, me.age);

    let serialized_user = serde_json::to_string(&me).unwrap();

    println!("{:?}", serialized_user);
}

#[derive(Serialize, Deserialize, Debug)]
struct Person {
    name: String,
    age: i32,
}

trait Greeter {
    fn greet(&self) -> ();
}

impl Greeter for Person {
    fn greet(&self) -> () {
        println!("Hellllllooooooo, I'm {}", self.name);
    }
}
