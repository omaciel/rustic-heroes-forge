// Enumerations for a Character

use std::collections::HashMap;
use std::error::Error;
use std::io;

use strum::IntoEnumIterator;

mod characters;

pub const BAR: [&str; 6] = [
    "Strength",
    "Dexterity", 
    "Constitution", 
    "Intelligence", 
    "Wisdom", 
    "Charisma",
];


#[derive(Debug)]
pub enum Race {
    Centaur,
    Changeling,
    Dwarf,
    Elf,
    Fairy,
    Gnome,
    Goblin,
    Human,
    Orc,
    Shifter,
    Zombie,
}

// Struct to hold character traits

pub struct Character {
    pub race: Race,
    pub class: characters::Class,
    pub abilities: HashMap<String, i32>,
}

impl Character {
    // Method to create a new character
    pub fn new() -> Self {
        Default::default()
    }
}

impl Default for Character {
    fn default() -> Self {
        let abilities: HashMap<String, i32> = HashMap::from([
            ("Strength".to_string(), 15),
            ("Dexterity".to_string(), 14),
            ("Constitution".to_string(), 13),
            ("Inteligence".to_string(), 12),
            ("Wisdom".to_string(), 10),
            ("Charisma".to_string(), 8),
        ]);
        Character {
            race: Race::Human,
            class: characters::Class::Paladin,
            abilities,
        }
    }
}

// Function to prompt user for a choice
fn prompt() -> i32 {
    let mut choice: String = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Could not parse your choice");

    let input: i32 =choice.trim().parse().expect("You provided an invalid input");
    input
}

// Function to prompt the user for character traits
pub fn run() -> Result<Character, Box<dyn Error>> {
    let mut character = Character::new();

    // Prompt for race
    println!("Choose a race:");
    println!("1. Human");
    println!("2. Elf");
    println!("3. Dwarf");
    
    match prompt() {
        1 => character.race = Race::Human,
        2 => character.race = Race::Elf,
        3 => character.race = Race::Dwarf,
        _ => println!("Invalid choice. Defaulting to Human."),
    }

    // Prompt for class
    println!("Choose a class:");
    println!("1. Paladin");
    println!("2. Spellcaster");
    println!("3. Rogue");
    
    match prompt() {
        1 => character.class = characters::Class::Paladin,
        2 => character.class = characters::Class::Sourcerer,
        3 => character.class = characters::Class::Rogue,
        _ => println!("Invalid choice. Defaulting to Paladin."),
    }

    // Prompt for abilities
    let mut idx: i32 = 0;
    for ab in characters::Class::iter() {
    
        idx += 1;
        println!("{}. {:?}", idx.to_string(), ab)
    }

    let mut abilities: HashMap<String, i32> = HashMap::new();
    for ab in BAR {
        let mut ability_score: String = String::new();
        println!("Enter ability scores for {}:", ab.to_string());
        io::stdin().read_line(&mut ability_score)?;
        let score: i32 = ability_score.trim().parse().unwrap();
        println!("{}: {}", ab.to_string(), score);
        abilities.insert(ab.to_string(), score);
    }
    character.abilities = abilities;

    Ok(character)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_character() {
        let character = Character::new();
        assert!(matches!(character.race, Race::Human));
        assert!(matches!(character.class, characters::Class::Paladin));
    }

    #[test]
    fn test_default_abilities() {
        let character = Character::new();
        assert!(matches!(character.race, Race::Human));
        assert!(matches!(character.class, characters::Class::Paladin));
    }

    #[test]
    fn test_set_race() {
        let abilities: HashMap<String, i32> = HashMap::from([
            ("Strength".to_string(), 15),
        ]);
        let character: Character = Character{
            race: Race::Dwarf, 
            class: characters::Class::Sourcerer, 
            abilities: abilities,
        };
        assert!(matches!(character.race, Race::Dwarf));
        assert!(matches!(character.class, characters::Class::Sourcerer));
    }
}