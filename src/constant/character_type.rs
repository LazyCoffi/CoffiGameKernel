pub enum Gender {
    Female,
    Male
}

pub enum Race {
    Dwarf,
    Elf,
    HalfElf,
    HumanBeing,
    Orc,
    ShadowBorn
}

#[derive(Eq, Hash, PartialEq)]
pub enum RoleTag {
    Archer,
    ArmoredWarrior,
    Assassin,
    BlackSmith,
    ElementWizard,
    Healer,
    Mage,
    Priest,
    ScaleKnight,
    Thief,
    WanderKnight,
}