use clap::{command, Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(author, version, about)]
pub struct Register {
    #[command(subcommand)]
    pub creature_type: CreatureType,
}

#[derive(Debug, Subcommand)]
pub enum CreatureType {
    /// Create a person register
    Person(PersonSubcommand),
    /// Create a pet register
    Pet(PetSubcommand),
}

#[derive(Debug, Args)]
pub struct PersonSubcommand {
    /// First name of the person
    #[arg(short = 'f', aliases = ["fn", "fname", "first_name"], long)]
    firstname: String,
    /// Last name of the person
    #[arg(short = 'l', aliases = ["ln", "lname", "last_name"], long)]
    lastname: String,
    /// Age of the person
    #[arg(short = 'a', aliases = ["age"], long, default_value = "45")]
    age: usize,
}

#[derive(Debug, Args)]
pub struct PetSubcommand {
    /// Name of the pet
    #[arg(short = 'n', aliases = ["pn", "pet_name", "petname"], long)]
    name: String,
}
