// use clap::{command, Arg, ArgGroup, Command};
use clap::{command, Arg, Command};

fn main() {
    // Positional arguments do not specify short and long methods
    // let _match_result = command!()
    //     .about("\n\nApplication to register creatures into the Fisrt Nations World")
    //     .group(
    //         ArgGroup::new("person-register")
    //             .arg("lastname")
    //             .arg("firstname")
    //             .conflicts_with("animal-register"),
    //     )
    //     .group(ArgGroup::new("animal-register").arg("pet-name"))
    //     .arg(
    //         Arg::new("firstname")
    //             .group("person-register")
    //             .short('f')
    //             .long("first-name")
    //             .aliases(["fname", "firstname", "fn"])
    //             .required(true)
    //             .help("First name"), // .conflicts_with("lastname"), to flag conflicts
    //     )
    //     .arg(
    //         Arg::new("lastname")
    //             .group("person-register")
    //             .short('l')
    //             .long("last-name")
    //             .aliases(["lname", "lastname", "ln"])
    //             .required(true)
    //             .help("Last name"),
    //     )
    //     .arg(
    //         Arg::new("pet-name")
    //             .group("animal-register")
    //             .long("pet-name")
    //             .short('n')
    //             .required(true),
    //     )
    //     .arg(
    //         Arg::new("fluffy")
    //             .long("fluffy")
    //             .required(false)
    //             .default_value("no")
    //             .help("Is the person using a fluffy coat or not"),
    //     )
    //     .get_matches();

    // Using subcommands instead of groups
    let match_result_2 = command!()
        .subcommand(
            Command::new("register-person")
                .arg(
                    Arg::new("firstname")
                        .short('f')
                        .long("first-name")
                        .aliases(["fname", "firstname", "fn"])
                        .required(true)
                        .help("First name"), // .conflicts_with("lastname"), to flag conflicts
                )
                .arg(
                    Arg::new("lastname")
                        .short('l')
                        .long("last-name")
                        .aliases(["lname", "lastname", "ln"])
                        .required(true)
                        .help("Last name"),
                ),
        )
        .subcommand(
            Command::new("register-animal").arg(
                Arg::new("pet-name")
                    .long("pet-name")
                    .short('n')
                    .required(true),
            ),
        )
        .about("\n\nApplication to register creatures into the Fisrt Nations World")
        .arg(
            // This command needs to be input in the CLI before the subcommands
            Arg::new("fluffy")
                .long("fluffy")
                .required(false)
                .default_value("no")
                .help("Is the person using a fluffy coat or not"),
        )
        .get_matches();

    // cargo run -- --fluffy true register-animal --pet-name Floppy
    println!(
        "Fluffy: {}",
        match_result_2.get_one::<String>("fluffy").unwrap()
    );
    println!(
        "Pet name: {}",
        match_result_2
            .subcommand_matches("register-animal")
            .unwrap()
            .get_one::<String>("pet-name")
            .unwrap()
    );
}
