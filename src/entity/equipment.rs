struct EquipmentAttr {
    cft_inc: u32,
    cft_dec: u32,
    con_inc: u32,
    con_dec: u32,
    dex_inc: u32,
    dex_dec: u32,
    int_inc: u32,
    int_dec: u32,
    hel_inc: u32,
    hel_dec: u32,
    ler_inc: u32,
    ler_dec: u32,
    per_inc: u32,
    per_dec: u32,
    spd_inc: u32,
    spd_dec: u32,
    stg_inc: u32,
    stg_dec: u32,
    wil_inc: u32,
    wil_dec: u32,
}

impl EquipmentAttr {
    pub fn set_cft_inc(&mut self, val: u32) {
        self.cft_dec = 0;
        self.cft_inc = val;
    }

    pub fn set_cft_dec(&mut self, val: u32) {
        self.cft_inc = 0;
        self.cft_dec = val;
    }

    pub fn get_cft_inc(&self) -> u32 {
        return self.cft_inc;
    }

    pub fn get_cft_dec(&self) -> u32 {
        return self.cft_dec;
    }

    pub fn set_con_inc(&mut self, val: u32) {
        self.con_dec = 0;
        self.con_inc = val;
    }

    pub fn set_con_dec(&mut self, val: u32) {
        self.con_inc = 0;
        self.con_dec = val;
    }

    pub fn get_con_inc(&self) -> u32 {
        return self.con_inc;
    }

    pub fn get_con_dec(&self) -> u32 {
        return self.con_dec;
    }

    pub fn set_dex_inc(&mut self, val: u32) {
        self.dex_dec = 0;
        self.dex_inc = val;
    }

    pub fn set_dex_dec(&mut self, val: u32) {
        self.dex_inc = 0;
        self.dex_dec = val;
    }

    pub fn get_dex_inc(&self) -> u32 {
        return self.dex_inc;
    }

    pub fn get_dex_dec(&self) -> u32 {
        return self.dex_dec;
    }

    pub fn set_int_inc(&mut self, val: u32) {
        self.int_dec = 0;
        self.int_inc = val;
    }

    pub fn set_int_dec(&mut self, val: u32) {
        self.int_inc = 0;
        self.int_dec = val;
    }

    pub fn get_int_inc(&self) -> u32 {
        return self.int_inc;
    }

    pub fn get_int_dec(&self) -> u32 {
        return self.int_dec;
    }

    pub fn set_hel_inc(&mut self, val: u32) {
        self.hel_dec = 0;
        self.hel_inc = val;
    }

    pub fn set_hel_dec(&mut self, val: u32) {
        self.hel_inc = 0;
        self.hel_dec = val;
    }

    pub fn get_hel_inc(&self) -> u32 {
        return self.hel_inc;
    }

    pub fn get_hel_dec(&self) -> u32 {
        return self.hel_dec;
    }

    pub fn set_ler_inc(&mut self, val: u32) {
        self.ler_dec = 0;
        self.ler_inc = val;
    }

    pub fn set_ler_dec(&mut self, val: u32) {
        self.ler_inc = 0;
        self.ler_dec = val;
    }

    pub fn get_ler_inc(&self) -> u32 {
        return self.ler_inc;
    }

    pub fn get_ler_dec(&self) -> u32 {
        return self.ler_dec;
    }

    pub fn set_per_inc(&mut self, val: u32) {
        self.per_dec = 0;
        self.per_inc = val;
    }

    pub fn set_per_dec(&mut self, val: u32) {
        self.per_inc = 0;
        self.per_dec = val;
    }

    pub fn get_per_inc(&self) -> u32 {
        return self.per_inc;
    }

    pub fn get_per_dec(&self) -> u32 {
        return self.per_dec;
    }

    pub fn set_spd_inc(&mut self, val: u32) {
        self.spd_dec = 0;
        self.spd_inc = val;
    }

    pub fn set_spd_dec(&mut self, val: u32) {
        self.spd_inc = 0;
        self.spd_dec = val;
    }

    pub fn get_spd_inc(&self) -> u32 {
        return self.spd_inc;
    }

    pub fn get_spd_dec(&self) -> u32 {
        return self.spd_dec;
    }

    pub fn set_stg_inc(&mut self, val: u32) {
        self.stg_dec = 0;
        self.stg_inc = val;
    }

    pub fn set_stg_dec(&mut self, val: u32) {
        self.stg_inc = 0;
        self.stg_dec = val;
    }

    pub fn get_stg_inc(&self) -> u32 {
        return self.stg_inc;
    }

    pub fn get_stg_dec(&self) -> u32 {
        return self.stg_dec;
    }

    pub fn set_wil_inc(&mut self, val: u32) {
        self.wil_dec = 0;
        self.wil_inc = val;
    }

    pub fn set_wil_dec(&mut self, val: u32) {
        self.wil_inc = 0;
        self.wil_dec = val;
    }

    pub fn get_wil_inc(&self) -> u32 {
        return self.wil_inc;
    }

    pub fn get_wil_dec(&self) -> u32 {
        return self.wil_dec;
    }
}

struct EquipmentState {
    health_inc: u32,
    health_dec: u32,
    will_inc: u32,
    will_dec: u32,

    blunt_attack_inc: u32,
    blunt_attack_dec: u32,
    fire_attack_inc: u32,
    fire_attack_dec: u32,
    frost_attack_inc: u32,
    frost_attack_dec: u32,
    lighting_attack_inc: u32,
    lighting_attack_dec: u32,
    poison_attack_inc: u32,
    poison_attack_dec: u32,
    sharp_attack_inc: u32,
    sharp_attack_dec: u32,

    blunt_defence_inc: u32,
    blunt_defence_dec: u32,
    fire_defence_inc: u32,
    fire_defence_dec: u32,
    frost_defence_inc: u32,
    frost_defence_dec: u32,
    lighting_defence_inc: u32,
    lighting_defence_dec: u32,
    poison_defence_inc: u32, 
    poison_defence_dec: u32, 
    sharp_defence_inc: u32,
    sharp_defence_dec: u32,

    despair_resist_inc: u32,
    despair_resist_dec: u32,
    fire_resist_inc: u32,
    fire_resist_dec: u32,
    frost_resist_inc: u32,
    frost_resist_dec: u32,
    lighting_resist_inc: u32,
    lighting_resist_dec: u32,
    poison_resist_inc: u32,
    poison_resist_dec: u32,
}

impl EquipmentState {
    pub fn set_health_inc(&mut self, val: u32) {
        self.health_dec = 0;
        self.health_inc = val;
    }

    pub fn set_health_dec(&mut self, val: u32) {
        self.health_inc = 0;
        self.health_dec = val;
    }

    pub fn get_health_inc(&self) -> u32 {
        return self.health_inc;
    }   

    pub fn get_health_dec(&self) -> u32 {
        return self.health_dec;
    }

    pub fn set_will_inc(&mut self, val: u32) {
        self.will_dec = 0;
        self.will_inc = val;
    }

    pub fn set_will_dec(&mut self, val: u32) {
        self.will_inc = 0;
        self.will_dec = val;
    }

    pub fn get_will_inc(&self) -> u32 {
        return self.will_inc;
    }   

    pub fn get_will_dec(&self) -> u32 {
        return self.will_dec;
    }
}

impl EquipmentState {
    pub fn set_blunt_attack_inc(&mut self, val: u32) {
        self.blunt_attack_dec = 0;
        self.blunt_attack_inc = val;
    }

    pub fn set_blunt_attack_dec(&mut self, val: u32) {
        self.blunt_attack_inc = 0;
        self.blunt_attack_dec = val;
    }

    pub fn get_blunt_attack_inc(&self) -> u32 {
        return self.blunt_attack_inc;
    }   

    pub fn get_blunt_attack_dec(&self) -> u32 {
        return self.blunt_attack_dec;
    }

    pub fn set_fire_attack_inc(&mut self, val: u32) {
        self.fire_attack_dec = 0;
        self.fire_attack_inc = val;
    }

    pub fn set_fire_attack_dec(&mut self, val: u32) {
        self.fire_attack_inc = 0;
        self.fire_attack_dec = val;
    }

    pub fn get_fire_attack_inc(&self) -> u32 {
        return self.fire_attack_inc;
    }   

    pub fn get_fire_attack_dec(&self) -> u32 {
        return self.fire_attack_dec;
    }

    pub fn set_frost_attack_inc(&mut self, val: u32) {
        self.frost_attack_dec = 0;
        self.frost_attack_inc = val;
    }

    pub fn set_frost_attack_dec(&mut self, val: u32) {
        self.frost_attack_inc = 0;
        self.frost_attack_dec = val;
    }

    pub fn get_frost_attack_inc(&self) -> u32 {
        return self.frost_attack_inc;
    }   

    pub fn get_frost_attack_dec(&self) -> u32 {
        return self.frost_attack_dec;
    }

    pub fn set_lighting_attack_inc(&mut self, val: u32) {
        self.lighting_attack_dec = 0;
        self.lighting_attack_inc = val;
    }

    pub fn set_lighting_attack_dec(&mut self, val: u32) {
        self.lighting_attack_inc = 0;
        self.lighting_attack_dec = val;
    }

    pub fn get_lighting_attack_inc(&self) -> u32 {
        return self.lighting_attack_inc;
    }   

    pub fn get_lighting_attack_dec(&self) -> u32 {
        return self.lighting_attack_dec;
    }

    pub fn set_poison_attack_inc(&mut self, val: u32) {
        self.poison_attack_dec = 0;
        self.poison_attack_inc = val;
    }

    pub fn set_poison_attack_dec(&mut self, val: u32) {
        self.poison_attack_inc = 0;
        self.poison_attack_dec = val;
    }

    pub fn get_poison_attack_inc(&self) -> u32 {
        return self.poison_attack_inc;
    }   

    pub fn get_poison_attack_dec(&self) -> u32 {
        return self.poison_attack_dec;
    }

    pub fn set_sharp_attack_inc(&mut self, val: u32) {
        self.sharp_attack_dec = 0;
        self.sharp_attack_inc = val;
    }

    pub fn set_sharp_attack_dec(&mut self, val: u32) {
        self.sharp_attack_inc = 0;
        self.sharp_attack_dec = val;
    }

    pub fn get_sharp_attack_inc(&self) -> u32 {
        return self.sharp_attack_inc;
    }   

    pub fn get_sharp_attack_dec(&self) -> u32 {
        return self.sharp_attack_dec;
    }
}

impl EquipmentState {
    pub fn set_blunt_defence_inc(&mut self, val: u32) {
        self.blunt_defence_dec = 0;
        self.blunt_defence_inc = val;
    }

    pub fn set_blunt_defence_dec(&mut self, val: u32) {
        self.blunt_defence_inc = 0;
        self.blunt_defence_dec = val;
    }

    pub fn get_blunt_defence_inc(&self) -> u32 {
        return self.blunt_defence_inc;
    }   

    pub fn get_blunt_defence_dec(&self) -> u32 {
        return self.blunt_defence_dec;
    }

    pub fn set_fire_defence_inc(&mut self, val: u32) {
        self.fire_defence_dec = 0;
        self.fire_defence_inc = val;
    }

    pub fn set_fire_defence_dec(&mut self, val: u32) {
        self.fire_defence_inc = 0;
        self.fire_defence_dec = val;
    }

    pub fn get_fire_defence_inc(&self) -> u32 {
        return self.fire_defence_inc;
    }   

    pub fn get_fire_defence_dec(&self) -> u32 {
        return self.fire_defence_dec;
    }

    pub fn set_frost_defence_inc(&mut self, val: u32) {
        self.frost_defence_dec = 0;
        self.frost_defence_inc = val;
    }

    pub fn set_frost_defence_dec(&mut self, val: u32) {
        self.frost_defence_inc = 0;
        self.frost_defence_dec = val;
    }

    pub fn get_frost_defence_inc(&self) -> u32 {
        return self.frost_defence_inc;
    }   

    pub fn get_frost_defence_dec(&self) -> u32 {
        return self.frost_defence_dec;
    }

    pub fn set_lighting_defence_inc(&mut self, val: u32) {
        self.lighting_defence_dec = 0;
        self.lighting_defence_inc = val;
    }

    pub fn set_lighting_defence_dec(&mut self, val: u32) {
        self.lighting_defence_inc = 0;
        self.lighting_defence_dec = val;
    }

    pub fn get_lighting_defence_inc(&self) -> u32 {
        return self.lighting_defence_inc;
    }   

    pub fn get_lighting_defence_dec(&self) -> u32 {
        return self.lighting_defence_dec;
    }

    pub fn set_poison_defence_inc(&mut self, val: u32) {
        self.poison_defence_dec = 0;
        self.poison_defence_inc = val;
    }

    pub fn set_poison_defence_dec(&mut self, val: u32) {
        self.poison_defence_inc = 0;
        self.poison_defence_dec = val;
    }

    pub fn get_poison_defence_inc(&self) -> u32 {
        return self.poison_defence_inc;
    }   

    pub fn get_poison_defence_dec(&self) -> u32 {
        return self.poison_defence_dec;
    }

    pub fn set_sharp_defence_inc(&mut self, val: u32) {
        self.sharp_defence_dec = 0;
        self.sharp_defence_inc = val;
    }

    pub fn set_sharp_defence_dec(&mut self, val: u32) {
        self.sharp_defence_inc = 0;
        self.sharp_defence_dec = val;
    }

    pub fn get_sharp_defence_inc(&self) -> u32 {
        return self.sharp_defence_inc;
    }   

    pub fn get_sharp_defence_dec(&self) -> u32 {
        return self.sharp_defence_dec;
    }   
}

impl EquipmentState {
    pub fn set_despair_resist_inc(&mut self, val: u32) {
        self.despair_resist_dec = 0;
        self.despair_resist_inc = val;
    }

    pub fn set_despair_resist_dec(&mut self, val: u32) {
        self.despair_resist_inc = 0;
        self.despair_resist_dec = val;
    }

    pub fn get_despair_resist_inc(&self) -> u32 {
        return self.despair_resist_inc;
    }   

    pub fn get_despair_resist_dec(&self) -> u32 {
        return self.despair_resist_dec;
    }

    pub fn set_fire_resist_inc(&mut self, val: u32) {
        self.fire_resist_dec = 0;
        self.fire_resist_inc = val;
    }

    pub fn set_fire_resist_dec(&mut self, val: u32) {
        self.fire_resist_inc = 0;
        self.fire_resist_dec = val;
    }

    pub fn get_fire_resist_inc(&self) -> u32 {
        return self.fire_resist_inc;
    }   

    pub fn get_fire_resist_dec(&self) -> u32 {
        return self.fire_resist_dec;
    }

    pub fn set_frost_resist_inc(&mut self, val: u32) {
        self.frost_resist_dec = 0;
        self.frost_resist_inc = val;
    }

    pub fn set_frost_resist_dec(&mut self, val: u32) {
        self.frost_resist_inc = 0;
        self.frost_resist_dec = val;
    }

    pub fn get_frost_resist_inc(&self) -> u32 {
        return self.frost_resist_inc;
    }   

    pub fn get_frost_resist_dec(&self) -> u32 {
        return self.frost_resist_dec;
    }

    pub fn set_lighting_resist_inc(&mut self, val: u32) {
        self.lighting_resist_dec = 0;
        self.lighting_resist_inc = val;
    }

    pub fn set_lighting_resist_dec(&mut self, val: u32) {
        self.lighting_resist_inc = 0;
        self.lighting_resist_dec = val;
    }

    pub fn get_lighting_resist_inc(&self) -> u32 {
        return self.lighting_resist_inc;
    }   

    pub fn get_lighting_resist_dec(&self) -> u32 {
        return self.lighting_resist_dec;
    }

    pub fn set_poison_resist_inc(&mut self, val: u32) {
        self.poison_resist_dec = 0;
        self.poison_resist_inc = val;
    }

    pub fn set_poison_resist_dec(&mut self, val: u32) {
        self.poison_resist_inc = 0;
        self.poison_resist_dec = val;
    }

    pub fn get_poison_resist_inc(&self) -> u32 {
        return self.poison_resist_inc;
    }   

    pub fn get_poison_resist_dec(&self) -> u32 {
        return self.poison_resist_dec;
    }   
}