use rand::Rng;

use std::path::PathBuf;
use clap::Parser; //, Subcommand};

#[derive(Parser)]
#[command(name = "roll")]
#[command(version = "1.0")]
#[command(about = "Command line dice roller", long_about = None)]

struct Cli {
    dice: Option<String>,
    modifier_operator: Option<String>,
    modifier_value: Option<String>,
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,
}

fn main() {
    let cli = Cli::parse();

    if let Some(dice) = cli.dice.as_deref() {

        let dice = dice.to_lowercase();

        //parse out times to roll and sides

        let parts: Vec<&str> = dice.split('d').collect();
        let num_rolls = parts[0].parse::<i32>().unwrap_or(1);
        let num_sides = parts[1].parse::<i32>().unwrap_or(20);
        
        let modifier_operator = match cli.modifier_operator.as_deref() {
            Some(modifier_operator) => modifier_operator,
            None => "None",
        };

        let modifier_value = match cli.modifier_value.as_deref() {
            Some(modifier) => modifier,
            None => "None",
        };

        match cli.debug {
            0 => {}
            _ => {
                println!("Value for dice: {dice}");
                println!("Value for num_rolls {num_rolls}");
                println!("Value for num_sides: {num_sides}");
                println!("Modifier found: {modifier_operator}");
                println!("Modifier value found: {modifier_value}");

            }
        }

        let result = rand::thread_rng().gen_range(1..=20);

        println!("{}", result);


    }

    println!("Thanks for rolling!");
}
