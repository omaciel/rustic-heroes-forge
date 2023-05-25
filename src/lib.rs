// Enumerations for a Character

use std::error::Error;
use std::io;


#[derive(Debug)]
pub enum Race {
    Dwarf,
    Elf,
    Human,
}

#[derive(Debug)]
pub enum Class {
    Rogue,
    Spellcaster,
    Warrior,
}


// Struct to hold character traits
pub struct Character {
    pub race: Race,
    pub class: Class,
    pub ability_scores: Vec<i32>,
    pub personality_traits: Vec<String>,
}

impl Character {
    // Method to create a new character
    pub fn new() -> Self {
        Default::default()
    }
}

impl Default for Character {
    fn default() -> Self {
        Character {
            race: Race::Human,
            class: Class::Warrior,
            ability_scores: Vec::new(),
            personality_traits: Vec::new(),
        }
    }
}

// Function to prompt the user for character traits
pub fn run() -> Result<Character, Box<dyn Error>> {
    let mut character = Character::new();

    // Prompt for race
    println!("Choose a race:");
    println!("1. Human");
    println!("2. Elf");
    println!("3. Dwarf");
    let mut race_choice = String::new();
    io::stdin().read_line(&mut race_choice)?;
    match race_choice.trim().parse() {
        Ok(1) => character.race = Race::Human,
        Ok(2) => character.race = Race::Elf,
        Ok(3) => character.race = Race::Dwarf,
        _ => println!("Invalid choice. Defaulting to Human."),
    }

    // Prompt for class
    println!("Choose a class:");
    println!("1. Warrior");
    println!("2. Spellcaster");
    println!("3. Rogue");
    let mut class_choice = String::new();
    io::stdin().read_line(&mut class_choice)?;
    match class_choice.trim().parse() {
        Ok(1) => character.class = Class::Warrior,
        Ok(2) => character.class = Class::Spellcaster,
        Ok(3) => character.class = Class::Rogue,
        _ => println!("Invalid choice. Defaulting to Warrior."),
    }

    // Prompt for ability scores
    println!("Enter ability scores (separated by spaces):");
    let mut ability_scores = String::new();
    io::stdin().read_line(&mut ability_scores)?;
    let scores: Vec<i32> = ability_scores
        .split_whitespace()
        .map(|s| s.parse().unwrap_or(0))
        .collect();
    character.ability_scores = scores;

    // Prompt for personality traits
    println!("Enter personality traits (separated by commas):");
    let mut personality_traits = String::new();
    io::stdin().read_line(&mut personality_traits)?;
    character.personality_traits = personality_traits
        .split(',')
        .map(|s| s.trim().to_string())
        .collect();

    Ok(character)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_character() {
        let character = Character::new();
        assert!(matches!(character.race, Race::Human));
        assert!(matches!(character.class, Class::Warrior));
    }

    #[test]
    fn test_set_race() {
        let character: Character = Character{
            race: Race::Dwarf, 
            class: Class::Spellcaster, 
            ability_scores: Vec::new(), 
            personality_traits: Vec::new() 
        };
        assert!(matches!(character.race, Race::Dwarf));
        assert!(matches!(character.class, Class::Spellcaster));
    }
}