/// DEMO RS FILE

fn main() {
    let name = "People".to_string();
    println!("Hi {}!", name);

    println!("Hi {}!",say_hello(Person::BOY("PETI".to_string())));
}

enum Person {
    BOY(String),
    GIRL(String)
}

fn say_hello(person: Person) -> String {
    match person {
        Person::BOY(name) => name,
        Person::GIRL(name) => name
    }
}
