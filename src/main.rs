extern crate bcrypt;
use colored::*;

use bcrypt::{hash, DEFAULT_COST};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("\n{}", "Simple Bcrypt Encoder".green().bold().underline());
        println!("\nDeveloped by: {}", "Fabio Covolo Mazzo".blue().bold());
        println!("Version: {}", "0.1".yellow().bold());

        println!("\n{}\n",
                 "This application was created with a simple purpose - to assist my clients \
        in generating a Bcrypt hash without the need to resort to other tools or online utilities. \
        It's important to note that the number of Bcrypt rounds for SpringBoot security is 10, so if \
        you're generating for an application developed with it, try first with this number or refer \
        to the application's code or documentation.".white());

        println!("{}", "The license is clearly MIT, due to its simplicity. :P".red().bold());
        println!("Usage: {} <password> <cost>", args[0]);
        std::process::exit(1);
    }

    let password = &args[1];
    let cost: u32 = match args[2].parse() {
        Ok(n) => n,
        Err(_) => {
            eprintln!("Error: cost must be a number");
            std::process::exit(1);
        }
    };

    match hash(password, cost) {
        Ok(hashed_password) => println!("{}", hashed_password),
        Err(_) => {
            eprintln!("Error: unable to hash password");
            std::process::exit(1);
        }
    }
}
