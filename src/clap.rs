use clap::{Arg, Command};

pub fn try_clap() {
    let matches = Command::new("Wallet-CLI")
        .about("this is a testcase for clap")
        .subcommand(
            Command::new("register-user")
                .about("Register a user with first and last name")
                .arg(
                    Arg::new("firstname")
                        .short('f')
                        .long("first-name")
                        .help("The user's first name")
                        .required(true)
                )
                .arg(
                    Arg::new("lastname")
                        .short('l')
                        .long("last-name")
                        .aliases(["lname"])
                        .help("This argument takes the person's lastname")
                        .required(true)
                ),
        )
        .get_matches();

    if let Some(sub_m) = matches.subcommand_matches("register-user") {
        
        let first = sub_m.get_one::<String>("firstname").unwrap();
        let last = sub_m.get_one::<String>("lastname").unwrap();

        println!("First Name: {:?}, Last Name: {:?}", first, last);

    } else {
        println!("No subcommand was used.");
    }
}