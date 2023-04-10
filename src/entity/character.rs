use std::collections::HashSet;

use crate::constant::character_type;
use crate::entity::buff;

pub struct CharacterAttr {
    // Enum
    character_name: String,
    gender: character_type::Gender,
    race: character_type::Race,
    role_tags: HashSet<character_type::RoleTag>,

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

impl CharacterAttr {
    pub fn get_character_name(&self) -> &String{
        return &self.character_name;
    }

    pub fn set_character_name(&mut self, character_name: String) {
        self.character_name = character_name;
    }

    pub fn get_gender(&self) -> &character_type::Gender {
        return &self.gender;
    }

    pub fn set_gender_male(&mut self) {
        self.gender = character_type::Gender::Male;
    }

    pub fn set_gender_female(&mut self) {
        self.gender = character_type::Gender::Female;
    }

    pub fn get_race(&self) -> &character_type::Race {
        return &self.race;
    }

    pub fn set_race_dwarf(&mut self) {
        self.race = character_type::Race::Dwarf;
    }

    pub fn set_race_elf(&mut self) {
        self.race = character_type::Race::Elf;
    }

    pub fn set_race_halfelf(&mut self) {
        self.race = character_type::Race::HalfElf;
    }

    pub fn set_race_humanbeing(&mut self) {
        self.race = character_type::Race::HumanBeing;
    }

    pub fn set_race_orc(&mut self) {
        self.race = character_type::Race::Orc;
    }

    pub fn set_race_shadowborn(&mut self) {
        self.race = character_type::Race::ShadowBorn;
    }

    pub fn has_roletag(&self, role_tag: character_type::RoleTag) -> bool {
        if let None = self.role_tags.get(&role_tag) {
            return false;
        } else {
            return true;
        }
    }

    pub fn get_roletag_num(&self) -> usize {
        return self.role_tags.len();
    }

    fn del_roletag(&mut self, role_tag: character_type::RoleTag) {
        self.role_tags.remove(&role_tag);
    }

    pub fn del_roletag_archer(&mut self) {
        let role_tag = character_type::RoleTag::Archer;
        self.del_roletag(role_tag);
    }

    pub fn del_roletag_armorwarrior(&mut self) {
        let role_tag = character_type::RoleTag::ArmoredWarrior;
        self.del_roletag(role_tag);
    }

    pub fn del_roletag_assassin(&mut self) {
        let role_tag = character_type::RoleTag::Assassin;
        self.del_roletag(role_tag);
    }

    pub fn del_roletag_blacksmith(&mut self) {
        let role_tag = character_type::RoleTag::BlackSmith;
        self.del_roletag(role_tag);
    }

    pub fn del_roletag_elementwizard(&mut self) {
        let role_tag = character_type::RoleTag::ElementWizard;
        self.del_roletag(role_tag);
    }

    pub fn del_roletag_healer(&mut self) {
        let role_tag = character_type::RoleTag::Healer;
        self.del_roletag(role_tag);
    }

    pub fn del_roletag_mage(&mut self) {
        let role_tag = character_type::RoleTag::Healer;
        self.del_roletag(role_tag);
    }

    pub fn del_roletag_priest(&mut self) {
        let role_tag = character_type::RoleTag::Priest;
        self.del_roletag(role_tag);
    }

    pub fn del_roletag_scaleknight(&mut self) {
        let role_tag = character_type::RoleTag::ScaleKnight;
        self.del_roletag(role_tag);
    }

    pub fn del_roletag_thier(&mut self) {
        let role_tag = character_type::RoleTag::Thief;
        self.del_roletag(role_tag);
    }

    pub fn del_roletag_wanderknight(&mut self) {
        let role_tag = character_type::RoleTag::WanderKnight;
        self.del_roletag(role_tag);
    }

    fn add_roletag(&mut self, role_tag: character_type::RoleTag) {
        if let None = self.role_tags.get(&role_tag) {
            self.role_tags.insert(role_tag);
        }
    }

    pub fn add_roletag_archer(&mut self) {
        let role_tag = character_type::RoleTag::Archer;
        self.add_roletag(role_tag);
    }

    pub fn add_roletag_armorwarrior(&mut self) {
        let role_tag = character_type::RoleTag::ArmoredWarrior;
        self.add_roletag(role_tag);
    }

    pub fn add_roletag_assassin(&mut self) {
        let role_tag = character_type::RoleTag::Assassin;
        self.add_roletag(role_tag);
    }

    pub fn add_roletag_blacksmith(&mut self) {
        let role_tag = character_type::RoleTag::BlackSmith;
        self.add_roletag(role_tag);
    }

    pub fn add_roletag_elementwizard(&mut self) {
        let role_tag = character_type::RoleTag::ElementWizard;
        self.add_roletag(role_tag);
    }

    pub fn add_roletag_healer(&mut self) {
        let role_tag = character_type::RoleTag::Healer;
        self.add_roletag(role_tag);
    }

    pub fn add_roletag_mage(&mut self) {
        let role_tag = character_type::RoleTag::Healer;
        self.add_roletag(role_tag);
    }

    pub fn add_roletag_priest(&mut self) {
        let role_tag = character_type::RoleTag::Priest;
        self.add_roletag(role_tag);
    }

    pub fn add_roletag_scaleknight(&mut self) {
        let role_tag = character_type::RoleTag::ScaleKnight;
        self.add_roletag(role_tag);
    }

    pub fn add_roletag_thier(&mut self) {
        let role_tag = character_type::RoleTag::Thief;
        self.add_roletag(role_tag);
    }

    pub fn add_roletag_wanderknight(&mut self) {
        let role_tag = character_type::RoleTag::WanderKnight;
        self.add_roletag(role_tag);
    }
}

impl CharacterAttr {
    pub fn add_stg(&mut self, inc: u32) {
        self.stg += inc;
    }

    pub fn sub_stg(&mut self, dec: u32) {
        self.stg = if dec <= self.stg {self.stg - dec} else {0};
    }

    pub fn set_stg(&mut self, val: u32) {
        self.stg = val;
    }

    pub fn add_int(&mut self, inc: u32) {
        self.int += inc;
    }

    pub fn sub_int(&mut self, dec: u32) {
        self.int = if dec <= self.int {self.int - dec} else {0};
    }

    pub fn set_int(&mut self, val: u32) {
        self.int = val;
    }

    pub fn add_dex(&mut self, inc: u32) {
        self.dex += inc;
    }

    pub fn sub_dex(&mut self, dec: u32) {
        self.dex = if dec <= self.dex {self.dex - dec} else {0};
    }

    pub fn set_dex(&mut self, val: u32) {
        self.dex = val;
    }

    pub fn add_spd(&mut self, inc: u32) {
        self.spd += inc;
    }

    pub fn sub_spd(&mut self, dec: u32) {
        self.spd = if dec <= self.spd {self.spd - dec} else {0};
    }

    pub fn set_spd(&mut self, val: u32) {
        self.spd = val;
    }

    pub fn add_con(&mut self, inc: u32) {
        self.con += inc;
    }

    pub fn sub_con(&mut self, dec: u32) {
        self.con = if dec <= self.con {self.con - dec} else {0};
    }

    pub fn set_con(&mut self, val: u32) {
        self.con = val;
    }

    pub fn add_per(&mut self, inc: u32) {
        self.per += inc;
    }

    pub fn sub_per(&mut self, dec: u32) {
        self.per = if dec <= self.per {self.per - dec} else {0};
    }

    pub fn set_per(&mut self, val: u32) {
        self.per = val;
    }

    pub fn add_ler(&mut self, inc: u32) {
        self.ler += inc;
    }

    pub fn sub_ler(&mut self, dec: u32) {
        self.ler = if dec <= self.ler {self.ler - dec} else {0};
    }

    pub fn set_ler(&mut self, val: u32) {
        self.ler = val;
    }

    pub fn add_wil(&mut self, inc: u32) {
        self.wil += inc;
    }

    pub fn sub_wil(&mut self, dec: u32) {
        self.wil = if dec <= self.wil {self.wil - dec} else {0};
    }

    pub fn set_wil(&mut self, val: u32) {
        self.wil = val;
    }

    pub fn add_hel(&mut self, inc: u32) {
        self.hel += inc;
    }

    pub fn sub_hel(&mut self, dec: u32) {
        self.hel = if dec <= self.hel {self.hel - dec} else {0};
    }

    pub fn set_hel(&mut self, val: u32) {
        self.hel = val;
    }

    pub fn add_cft(&mut self, inc: u32) {
        self.cft += inc;
    }

    pub fn sub_cft(&mut self, dec: u32) {
        self.cft = if dec <= self.cft {self.cft - dec} else {0};
    }

    pub fn set_cft(&mut self, val: u32) {
        self.cft = val;
    }
}

struct CharacterState {
    health: u32,
    will: u32,

    blunt_attack: u32,
    fire_attack: u32,
    frost_attack: u32,
    lighting_attack: u32,
    poison_attack: u32,    
    sharp_attack: u32,

    blunt_defence: u32,
    fire_defence: u32,
    frost_defence: u32,
    lighting_defence: u32,
    poison_defence: u32, 
    sharp_defence: u32,

    despair_resist: u32,
    fire_resist: u32,
    frost_resist: u32,
    lighting_resist: u32,
    posison_resist: u32,

    buff_list: Vec<buff::Buff> 
}

impl CharacterState {
    pub fn get_health(&self) -> u32 { 
        return self.health;
    }

    pub fn get_will(&self) -> u32 { 
        return self.will;
    }

    pub fn set_health(&mut self, val: u32) { 
        self.health = val;
    }

    pub fn set_will(&mut self, val: u32) { 
        self.will = val;
    }
}

impl CharacterState {
    pub fn get_blunt_attack(&self) -> u32 { 
        return self.blunt_attack;
    }

    pub fn get_fire_attack(&self) -> u32 { 
        return self.fire_attack;
    }

    pub fn get_frost_attack(&self) -> u32 { 
        return self.frost_attack;
    }

    pub fn get_lighting_attack(&self) -> u32 { 
        return self.lighting_attack;
    }

    pub fn get_poison_attack(&self) -> u32 { 
        return self.poison_attack;
    }

    pub fn get_sharp_attack(&self) -> u32 { 
        return self.sharp_attack;
    }

    pub fn set_blunt_attack(&mut self, val: u32) { 
        self.blunt_attack = val;
    }

    pub fn set_fire_attack(&mut self, val: u32) { 
        self.fire_attack = val;
    }

    pub fn set_frost_attack(&mut self, val: u32) { 
        self.frost_attack = val;
    }

    pub fn set_lighting_attack(&mut self, val: u32) { 
        self.lighting_attack = val;
    }

    pub fn set_poison_attack(&mut self, val: u32) { 
        self.poison_attack = val;
    }

    pub fn set_sharp_attack(&mut self, val: u32) { 
        self.sharp_attack = val;
    }
}

impl CharacterState {
    pub fn get_blunt_defence(&self) -> u32 { 
        return self.blunt_defence;
    }

    pub fn get_fire_defence(&self) -> u32 { 
        return self.fire_defence;
    }

    pub fn get_frost_defence(&self) -> u32 { 
        return self.fire_defence;
    }

    pub fn get_lighting_defence(&self) -> u32 { 
        return self.lighting_defence;
    }

    pub fn get_poison_defence(&self) -> u32 { 
        return self.poison_defence;
    }

    pub fn get_sharp_defence(&self) -> u32 { 
        return self.sharp_defence;
    }

    pub fn set_blunt_defence(&mut self, val: u32) { 
        self.blunt_defence = val;
    }

    pub fn set_fire_defence(&mut self, val: u32) { 
        self.fire_defence = val;
    }

    pub fn set_frost_defence(&mut self, val: u32) { 
        self.fire_defence = val;
    }

    pub fn set_lighting_defence(&mut self, val: u32) { 
        self.lighting_defence = val;
    }

    pub fn set_poison_defence(&mut self, val: u32) { 
        self.poison_defence = val;
    }

    pub fn set_sharp_defence(&mut self, val: u32) { 
        self.sharp_defence = val;
    }
}

impl CharacterState {
    pub fn get_despair_resist(&self) -> u32 { 
        return self.despair_resist;
    }

    pub fn get_fire_resist(&self) -> u32 { 
        return self.fire_resist;
    }

    pub fn get_frost_resist(&self) -> u32 { 
        return self.frost_resist;
    }

    pub fn get_lighting_resist(&self) -> u32 { 
        return self.lighting_resist;
    }

    pub fn get_posison_resist(&self) -> u32 { 
        return self.posison_resist;
    }

    pub fn set_despair_resist(&mut self, val: u32) { 
        self.despair_resist = val;
    }

    pub fn set_fire_resist(&mut self, val: u32) { 
        self.fire_resist = val;
    }

    pub fn set_frost_resist(&mut self, val: u32) { 
        self.frost_resist = val;
    }

    pub fn set_lighting_resist(&mut self, val: u32) { 
        self.lighting_resist = val;
    }

    pub fn set_posison_resist(&mut self, val: u32) { 
        self.posison_resist = val;
    }
}

struct Character {
    attr: CharacterAttr,
    state: CharacterState
}