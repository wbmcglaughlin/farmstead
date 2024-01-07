#[derive(Debug, Clone, Copy)]
pub enum MaterialType {
    Wheat,
}

impl MaterialType {
    pub fn texture_index(&self) -> usize {
        match self {
            MaterialType::Wheat => 0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Material {
    pub material_type: MaterialType,
}
