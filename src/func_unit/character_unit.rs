use crate::data::general_vec_data;

pub struct Character {
    attr: general_vec_data::GeneralVec
}

impl Character {
    pub fn init_name(&mut self, name: String) {
        self.attr.push_str(String::from("name"), name);
    }

    pub fn init_hp(&mut self, hp: u32) {
        self.attr.push_uint(String::from("hp"), hp);
    }

    pub fn print_name(&self) {
        if let general_vec_data::GeneralType::String(name) =
            self.attr.peek_attr_by_name(String::from("name")) {
            println!("Character's name is {name}");
        }
    }

    pub fn print_hp(&self) {
        if let general_vec_data::GeneralType::UInt(hp) =
            self.attr.peek_attr_by_name(String::from("hp")) {
            println!("Character's name is {hp}");
        }
    }

    pub fn add_hp_by(&mut self, increment: u32) {
        self.attr.set_uint_by_name(String::from("name"), increment);
    }
}
