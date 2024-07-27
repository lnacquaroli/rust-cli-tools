use clap::Parser;

#[derive(Parser)]
struct Person {
    name: String,
    age: usize,
}

pub fn clapper() {
    let person = Person::parse();
    println!("{} is {} years old", person.name, person.age);
}
