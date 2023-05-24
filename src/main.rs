use std::io;
use rustic_heroes_forge::Character;
use rustic_heroes_forge::Race;

// Function to prompt the user for character traits
fn prompt_character_creation() -> Character {
    let mut character = Character::new();

    // Prompt for race
    println!("Choose a race:");
    println!("1. Human");
    println!("2. Elf");
    println!("3. Dwarf");
    let mut race_choice = String::new();
    io::stdin().read_line(&mut race_choice).expect("Failed to read line");
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
    io::stdin().read_line(&mut class_choice).expect("Failed to read line");
    match class_choice.trim().parse() {
        Ok(1) => character.class = String::from("Warrior"),
        Ok(2) => character.class = String::from("Spellcaster"),
        Ok(3) => character.class = String::from("Rogue"),
        _ => println!("Invalid choice. Defaulting to Warrior."),
    }

    // Prompt for ability scores
    println!("Enter ability scores (separated by spaces):");
    let mut ability_scores = String::new();
    io::stdin().read_line(&mut ability_scores).expect("Failed to read line");
    let scores: Vec<i32> = ability_scores
        .split_whitespace()
        .map(|s| s.parse().unwrap_or(0))
        .collect();
    character.ability_scores = scores;

    // Prompt for personality traits
    println!("Enter personality traits (separated by commas):");
    let mut personality_traits = String::new();
    io::stdin().read_line(&mut personality_traits).expect("Failed to read line");
    character.personality_traits = personality_traits
        .split(',')
        .map(|s| s.trim().to_string())
        .collect();

    character
}

fn main() {
    let character = prompt_character_creation();
    println!("Character created:");
    println!("Race: {:?}", character.race);
    println!("Class: {}", character.class);
    println!("Ability Scores: {:?}", character.ability_scores);
    println!("Personality Traits: {:?}", character.personality_traits);
}
