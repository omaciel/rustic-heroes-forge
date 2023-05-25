use std::process;
use rustic_heroes_forge::run;


fn main() {
    match run() {
        Ok(character) => {
            println!("Character created:");
            println!("Race: {:?}", character.race);
            println!("Class: {:?}", character.class);
            println!("Ability Scores: {:?}", character.ability_scores);
            println!("Personality Traits: {:?}", character.personality_traits);
        },
        Err(e) => {
            eprintln!("Application error: {e}");
            process::exit(1);
        }
    }
}
