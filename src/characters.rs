use strum_macros::EnumIter;

#[derive(Debug, EnumIter)]
pub enum Class {
    Barbarian,
    Bard,
    Cleric,
    Druid,
    Fighter,
    Monk,
    Paladin,
    Range,
    Rogue,
    Sourcerer,
    Warlock,
    Wizard,
}
