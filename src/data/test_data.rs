pub struct TestData {
    hp: u32,
    stg: u32
}

impl TestData {
    pub fn new() -> TestData{
        TestData {
            hp: 0u32,
            stg: 0u32
        }
    }

    pub fn hp_zero(&mut self) {
        self.hp = 0u32;
    }

    pub fn hp_inc(&mut self) {
        self.hp += 1u32;
    }

    pub fn get_hp(&self) -> u32 {
        self.hp
    }

    pub fn stg_zero(&mut self) {
        self.stg = 0u32;
    }

    pub fn stg_inc(&mut self) {
        self.stg += 1;
    }

    pub fn get_stg(&self) -> u32 {
        self.stg
    }
}
