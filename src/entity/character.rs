use crate::constant::character_type;
pub struct CharacterAttr {
    // Enum
    character_name: String,
    gender: character_type::Gender,
    race: character_type::Race,
    role_tags: Vec<character_type::RoleTag>,
    personality_tags: Vec<character_type::PersonalityTag>,

    // Attr
    stg: u32,   // 力量
    int: u32,   // 智力
    dex: u32,   // 敏捷
    spd: u32,   // 速度
    con: u32,   // 体质
    per: u32,   // 感知
    ler: u32,   // 学识
    wil: u32,   // 意志
    hel: u32,   // 疗愈
    cft: u32,   // 制造
}