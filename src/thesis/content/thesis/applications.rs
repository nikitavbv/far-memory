use crate::thesis::engine::{Block, ApplicationBlock};

pub const COMPONENT_DIAGRAM_SYSTEM: &str = "component_diagram_system";

pub fn applications() -> Block {
    Block::Multiple(vec![
        Block::Application(ApplicationBlock::new(COMPONENT_DIAGRAM_SYSTEM)),
    ])
}
