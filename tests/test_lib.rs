use rustic_heroes_forge::Character;
use rustic_heroes_forge::Race;

#[test]
fn test_default_character() {
    let character = Character::new();
    assert!(matches!(character.race, Race::Human))
}