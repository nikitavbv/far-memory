use crate::thesis::engine::{Block, ApplicationBlock};

pub const DEPLOYMENT_DIAGRAM: &str = "deployment_diagram";
pub const COMPONENT_DIAGRAM: &str = "component_diagram";

pub fn applications() -> Block {
    Block::Multiple(vec![
        Block::Application(ApplicationBlock::new(DEPLOYMENT_DIAGRAM)), // this should be a deployment diagram - show components and how they communicate
        Block::Application(ApplicationBlock::new(COMPONENT_DIAGRAM)), // this should be a component diagram - show which data is passed where
    ])
}
