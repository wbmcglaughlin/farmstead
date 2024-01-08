#[derive(Debug, Clone, Copy)]
pub enum ResourceType {
    Tree,
    Stone,
}

impl ResourceType {
    pub fn file_path(&self) -> String {
        let filename = match self {
            ResourceType::Tree => "tree.png",
            ResourceType::Stone => todo!(),
        };

        String::from("sprites/resources/") + filename
    }
}
