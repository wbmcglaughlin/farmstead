#[derive(Debug, Clone, Copy)]
pub enum MaterialType {
    Wheat,
}

#[derive(Debug, Clone)]
pub struct Material {
    pub material_type: MaterialType,
}
