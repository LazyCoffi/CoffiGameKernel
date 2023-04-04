pub enum GeneralType {
    Char(i8),
    Short(i16),
    Int(i32),
    Long(i64),
    UChar(u8),
    UShort(u16),
    UInt(u32),
    ULong(u64),
    Float(f32),
    Double(f64),
    String(String),
    None
}

pub struct GeneralVec {
    vec: Vec<(String, GeneralType)>,
}

impl GeneralVec {
    pub fn new() -> GeneralVec{
        GeneralVec {
            vec: Vec::new()
        }
    }

    pub fn push_char(&mut self, attr_name: String, attr: i8) {
        self.vec.push((attr_name, GeneralType::Char(attr)));
    }

    pub fn push_short(&mut self, attr_name: String, attr: i16) {
        self.vec.push((attr_name, GeneralType::Short(attr)));
    }

    pub fn push_int(&mut self, attr_name: String, attr: i32) {
        self.vec.push((attr_name, GeneralType::Int(attr)));
    }

    pub fn push_long(&mut self, attr_name: String, attr: i64) {
        self.vec.push((attr_name, GeneralType::Long(attr)));
    }

    pub fn push_uchar(&mut self, attr_name: String, attr: u8) {
        self.vec.push((attr_name, GeneralType::UChar(attr)));
    }

    pub fn push_ushort(&mut self, attr_name: String, attr: u16) {
        self.vec.push((attr_name, GeneralType::UShort(attr)));
    }

    pub fn push_uint(&mut self, attr_name: String, attr: u32) {
        self.vec.push((attr_name, GeneralType::UInt(attr)));
    }

    pub fn push_ulong(&mut self, attr_name: String, attr: u64) {
        self.vec.push((attr_name, GeneralType::ULong(attr)));
    }

    pub fn push_float(&mut self, attr_name: String, attr: f32) {
        self.vec.push((attr_name, GeneralType::Float(attr)));
    }

    pub fn push_double(&mut self, attr_name: String, attr: f64) {
        self.vec.push((attr_name, GeneralType::Double(attr)));
    }

    pub fn push_str(&mut self, attr_name: String, attr: String) {
        self.vec.push((attr_name, GeneralType::String(attr)))
    }

    pub fn push_none(&mut self, attr_name: String) {
        self.vec.push((attr_name, GeneralType::None))
    }

    pub fn remove_by_index(&mut self, index: usize) -> GeneralType{
        if index >= self.vec.len() {
            return GeneralType::None;
        }

        return self.vec.remove(index).1;
    }

    pub fn remove_by_name(&mut self, attr_name: String) -> GeneralType{
        if let Some(x) = self.vec.iter().position(|x| *x.0 == attr_name) {
            return self.vec.remove(x).1;
        } else {
            return GeneralType::None;
        }
    }

    pub fn del_by_index(&mut self, index: usize) {
        if index < self.vec.len() {
            self.vec.remove(index);
        }
    }

    pub fn del_by_name(&mut self, attr_name: String) {
        self.vec.retain(|x| x.0 == attr_name);
    }

    pub fn peek_attr_by_index(&self, index: usize) -> &GeneralType{
        if index >= self.vec.len() {
            return &GeneralType::None;
        }

        return &self.vec[index].1;
    }

    pub fn peek_attr_by_name(&self, attr_name: String) -> &GeneralType{
        if let Some(x) = self.vec.iter().position(|x| *x.0 == attr_name) {
            return &self.vec[x].1;
        } else {
            return &GeneralType::None;
        }
    }

    pub fn set_char_by_index(&mut self, index: usize, val: i8) {
        if index < self.vec.len() {
            self.vec[index] = (self.vec[index].0.clone(), GeneralType::Char(val));
        }
    }

    pub fn set_char_by_name(&mut self, attr_name: String, val: i8) {
        if let Some(x) = self.vec.iter().position(|x| *x.0 == attr_name) {
            self.vec[x] = (self.vec[x].0.clone(), GeneralType::Char(val));
        }
    }

    pub fn set_short_by_index(&mut self, index: usize, val: i16) {
        if index < self.vec.len() {
            self.vec[index] = (self.vec[index].0.clone(), GeneralType::Short(val));
        }
    }

    pub fn set_short_by_name(&mut self, attr_name: String, val: i16) {
        if let Some(x) = self.vec.iter().position(|x| *x.0 == attr_name) {
            self.vec[x] = (self.vec[x].0.clone(), GeneralType::Short(val));
        }
    }

    pub fn set_int_by_index(&mut self, index: usize, val: i32) {
        if index < self.vec.len() {
            self.vec[index] = (self.vec[index].0.clone(), GeneralType::Int(val));
        }
    }

    pub fn set_int_by_name(&mut self, attr_name: String, val: i32) {
        if let Some(x) = self.vec.iter().position(|x| *x.0 == attr_name) {
            self.vec[x] = (self.vec[x].0.clone(), GeneralType::Int(val));
        }
    }

    pub fn set_long_by_index(&mut self, index: usize, val: i64) {
        if index < self.vec.len() {
            self.vec[index] = (self.vec[index].0.clone(), GeneralType::Long(val));
        }
    }

    pub fn set_long_by_name(&mut self, attr_name: String, val: i64) {
        if let Some(x) = self.vec.iter().position(|x| *x.0 == attr_name) {
            self.vec[x] = (self.vec[x].0.clone(), GeneralType::Long(val));
        }
    }

    pub fn set_uchar_by_index(&mut self, index: usize, val: u8) {
        if index < self.vec.len() {
            self.vec[index] = (self.vec[index].0.clone(), GeneralType::UChar(val));
        }
    }

    pub fn set_uchar_by_name(&mut self, attr_name: String, val: u8) {
        if let Some(x) = self.vec.iter().position(|x| *x.0 == attr_name) {
            self.vec[x] = (self.vec[x].0.clone(), GeneralType::UChar(val));
        }
    }

    pub fn set_ushort_by_index(&mut self, index: usize, val: u16) {
        if index < self.vec.len() {
            self.vec[index] = (self.vec[index].0.clone(), GeneralType::UShort(val));
        }
    }

    pub fn set_ushort_by_name(&mut self, attr_name: String, val: u16) {
        if let Some(x) = self.vec.iter().position(|x| *x.0 == attr_name) {
            self.vec[x] = (self.vec[x].0.clone(), GeneralType::UShort(val));
        }
    }

    pub fn set_uint_by_index(&mut self, index: usize, val: u32) {
        if index < self.vec.len() {
            self.vec[index] = (self.vec[index].0.clone(), GeneralType::UInt(val));
        }
    }

    pub fn set_uint_by_name(&mut self, attr_name: String, val: u32) {
        if let Some(x) = self.vec.iter().position(|x| *x.0 == attr_name) {
            self.vec[x] = (self.vec[x].0.clone(), GeneralType::UInt(val));
        }
    }

    pub fn set_ulong_by_index(&mut self, index: usize, val: u64) {
        if index < self.vec.len() {
            self.vec[index] = (self.vec[index].0.clone(), GeneralType::ULong(val));
        }
    }

    pub fn set_ulong_by_name(&mut self, attr_name: String, val: u64) {
        if let Some(x) = self.vec.iter().position(|x| *x.0 == attr_name) {
            self.vec[x] = (self.vec[x].0.clone(), GeneralType::ULong(val));
        }
    }

    pub fn set_str_by_index(&mut self, index: usize, val: String) {
        if index < self.vec.len() {
           self.vec[index] = (self.vec[index].0.clone(), GeneralType::String(val));
        }
    }

    pub fn set_str_by_name(&mut self, attr_name: String, val: String) {
        if let Some(x) = self.vec.iter().position(|x| *x.0 == attr_name) {
            self.vec[x] = (self.vec[x].0.clone(), GeneralType::String(val));
        }
    }

    pub fn set_none_by_index(&mut self, index: usize) {
        if index < self.vec.len() {
           self.vec[index] = (self.vec[index].0.clone(), GeneralType::None);
        }
    }

    pub fn set_none_by_name(&mut self, attr_name: String) {
        if let Some(x) = self.vec.iter().position(|x| *x.0 == attr_name) {
            self.vec[x] = (self.vec[x].0.clone(), GeneralType::None);
        }
    }

    pub fn get_name_by_index(&self, index: usize) -> &String{
        return &self.vec[index].0;
    }

    pub fn get_index_by_name(&self, attr_name: String) -> usize{
        if let Some(x) = self.vec.iter().position(|x| *x.0 == attr_name) {
            return x;
        } else {
            return usize::MAX;
        }
    }
}
