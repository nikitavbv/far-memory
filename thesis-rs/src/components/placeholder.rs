use {
    tracing::warn,
    docx_rs::{Paragraph, Run},
};

pub trait PlaceholderComponent {
    fn add_placeholder_component(self, text: &str, description: &str) -> Self;
}

impl PlaceholderComponent for Paragraph {
    fn add_placeholder_component(self, text: &str, description: &str) -> Self {
        warn!("Adding placeholder with description \"{}\"", description);
        
        self.add_run(Run::new()
            .highlight("yellow")
            .add_text(text))
    }
}