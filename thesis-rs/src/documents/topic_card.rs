use docx_rs::Docx;

pub trait TopicCardDocument {
    fn add_topic_card_document(self) -> Self;
}

impl TopicCardDocument for Docx {
    fn add_topic_card_document(self) -> Self {
        self
    }
}