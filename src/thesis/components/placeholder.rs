use {
    tracing::warn,
    docx_rs::{Paragraph, Run},
};

pub trait PlaceholderComponent {
    fn add_placeholder_component(self, text: impl Into<String>, description: impl Into<String>) -> Self;
}

impl PlaceholderComponent for Paragraph {
    fn add_placeholder_component(self, text: impl Into<String>, description: impl Into<String>) -> Self {
        warn!("Adding placeholder with description \"{}\"", description.into());
        
        self.add_run(Run::new()
            .highlight("yellow")
            .add_text(text))
    }
}