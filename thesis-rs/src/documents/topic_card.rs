use docx_rs::{Docx, Paragraph, Run, BreakType, AlignmentType};

pub trait TopicCardDocument {
    fn add_topic_card_document(self) -> Self;
}

impl TopicCardDocument for Docx {
    fn add_topic_card_document(self) -> Self {
        self
            .add_paragraph(Paragraph::new()
            .add_run(Run::new()
                .size(28)
                .bold()
                .add_text("Національний технічний університет України".to_uppercase())
                .add_break(BreakType::TextWrapping)
                .add_text("«Київський Політехнічний Інститут".to_uppercase())
                .add_break(BreakType::TextWrapping)
                .add_text("імені ")
                .add_text("Ігоря Сікорського»".to_uppercase())
            )
            .align(AlignmentType::Center))
    }
}