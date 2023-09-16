pub enum Block {
    SectionHeader(String),
    SubsectionHeader(String),
    Paragraph(String),
    UnorderedList(Vec<String>),
    Image(ImageBlock),
}

pub struct ImageBlock {
    path: String,
    description: String,
}

impl ImageBlock {
    pub fn new(path: String, description: String) -> Self {
        Self {
            path,
            description,
        }
    }

    pub fn path(&self) -> String {
        self.path.clone()
    }

    pub fn description(&self) -> String {
        self.description.clone()
    }
}