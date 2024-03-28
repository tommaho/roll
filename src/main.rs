use rand::Rng;
use std::fmt::{self, write};

//use std::path::PathBuf;
use clap::Parser; //, Subcommand};

#[derive(Parser)]
#[command(name = "roll")]
#[command(version = "1.0")]
#[command(about = "Command line dice roller", long_about = None)]

#[derive(Debug)]
struct Cli {
    dice: Option<String>,
    modifier_operator: Option<String>,
    modifier_value: Option<String>,
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,
}

struct Dice{
    sides: u32,
    roll: u32,
}
impl Dice {
    fn new(sides: u32) -> Dice {
        let roll = Dice::roll(sides);
        Dice {
            sides, roll}
    }
    fn roll(sides: u32) -> u32{
        rand::thread_rng().gen_range(1..=sides)
    }
}
impl fmt::Display for Dice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Dice {{ sides: {} roll: {}}} ", self.sides, self.roll)
    }
}

fn main() {

    let cli = Cli::parse();  

    //debug
    //println!("{:?}", cli);

    //default to 1d20
    let roll = cli.dice.as_deref().unwrap_or("1d20").to_lowercase();

    let (rolls, sides) = parse_dice(&roll);

    // I should take rolls and sides, the create a vector of dice
    // from that

    let mut dice_rolls = Vec::new();
    let mut sum_of_rolls = 0;

    for _ in 0..rolls {
        let dice = Dice::new(sides);
        sum_of_rolls += dice.roll;
        dice_rolls.push(dice);
    }

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
            println!("DEBUG: Value for sides: {sides}");
            println!("DEBUG: Value for num_rolls: {rolls}");
            for dice in &dice_rolls {
                println!("DEBUG: {}", dice);
            }
            //println!("Value for num_sides: {sides}");
            println!("DEBUG: Modifier found: {modifier_operator}");
            println!("DEBUG: Modifier value found: {modifier_value}");

        }
    }

    for (index, dice) in dice_rolls.iter().enumerate() {
        println!("Roll {}: {}", index + 1, dice.roll);
    }

    println!("Sum of rolls: {}", sum_of_rolls);
}

fn parse_dice(dice: &str) -> (u32, u32) {

    let parts: Vec<&str> = dice.split('d').collect();
    let rolls = parts[0].parse::<u32>().unwrap_or(1);
    let sides = parts[1].parse::<u32>().unwrap_or(20);
    
    // old code
    // Dice::new(sides)
    
    (rolls, sides)
    
}