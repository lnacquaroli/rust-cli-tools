use clap::Parser;

#[derive(Parser)]
struct Person {
    /// Name of the person
    #[arg(short = 'n', long, default_value_t = String::from("Jennie"))]
    name: String,
    /// Age of the person
    #[arg(short = 'a', long)]
    age: usize,
}

pub fn learning_clap() {
    let person = Person::parse();
    println!("{} is {} years old.", person.name, person.age);
}
