// Enumerations for a Character

#[derive(Debug)]
pub enum Race {
    Dwarf,
    Elf,
    Human,
}


// Struct to hold character traits
pub struct Character {
    pub race: Race,
    pub class: String,
    pub ability_scores: Vec<i32>,
    pub personality_traits: Vec<String>,
}

impl Character {
    // Method to create a new character
    pub fn new() -> Character {
        Character {
            race: Race::Human,
            class: String::new(),
            ability_scores: Vec::new(),
            personality_traits: Vec::new(),
        }
    }
}

impl Default for Character {
    fn default() -> Self {
        Character::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_character() {
        let character = Character::new();
        assert!(matches!(character.race, Race::Human))
    }
}