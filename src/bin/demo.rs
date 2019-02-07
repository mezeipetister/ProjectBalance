// Copyright (C) 2019 by Peter Mezei

enum Human {
    BOY,
    GIRL,
}

#[derive(Debug)]
enum Colors {
    RED = 1,
    GREEN = 2,
}

fn main() {
    let items = [1, 2, 3, 4, 5];
    for item in &items {
        println!("{}", item);
    }

    for i in items.iter() {
        println!("{}", i);
    }

    let amelie = Human::GIRL;
    get_human_sex(amelie);
    println!("{:?}", Colors::GREEN);
}

fn get_human_sex(human: Human) {
    match human {
        Human::BOY => println!("It's a boy!"),
        Human::GIRL => println!("It's a girl!"),
    }
}
