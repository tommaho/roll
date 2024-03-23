use rand::Rng;

use std::path::PathBuf;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "roll")]
#[command(version = "1.0")]
#[command(about = "Command line dice roller", long_about = None)]

struct Cli {
    dice: Option<String>,
    modifier_operator: Option<String>,
    modifier_value: Option<String>,

}

fn main() {
    let cli = Cli::parse();

    if let Some(dice) = cli.dice.as_deref() {
        println!("Value for dice: {dice}");
    }

    if let Some(modifier_operator) = cli.modifier_operator.as_deref() {
        println!("Modifier found: {modifier_operator}");
    }

    if let Some(modifier_value) = cli.modifier_value.as_deref() {
        println!("Modifier value found: {modifier_value}");
    }  

    let result = rand::thread_rng().gen_range(1..=20);

    println!("{}", result);
}
