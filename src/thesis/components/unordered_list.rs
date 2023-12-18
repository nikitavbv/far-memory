use {
    docx_rs::{
        Docx,
        Paragraph,
        AbstractNumbering,
        Level,
        Start,
        NumberFormat,
        LevelText,
        LevelJc,
        SpecialIndentType,
        Numbering,
        Tab,
        LineSpacing,
        AlignmentType,
        NumberingId,
        Run,
        IndentLevel,
    },
    crate::thesis::context::Context,
};

pub trait UnorderedListComponent {
    fn add_unordered_list_component(self, context: &mut Context, list: Vec<String>) -> Self;
}

impl UnorderedListComponent for Docx {
    fn add_unordered_list_component(self, context: &mut Context, list: Vec<String>) -> Self {
        let numbering_id = context.next_numbering_id();

        let mut document = self
            .add_abstract_numbering(
                AbstractNumbering::new(numbering_id)
                    .add_level(Level::new(
                        0,
                        Start::new(0),
                        NumberFormat::new("bullet"),
                        LevelText::new("– "),
                        LevelJc::new("left")
                    ).indent(None, Some(SpecialIndentType::FirstLine(725)), None, None))
            )
            .add_numbering(Numbering::new(numbering_id, numbering_id));

        for i in 0..list.len() {
            let text = list.get(i).unwrap().clone();
            let text = if !text.ends_with("?") {
                format!("{}{}", text, if i == list.len() - 1 { "." } else { ";" })
            } else {
                text
            };

            document = document.add_paragraph(
                Paragraph::new()
                    .add_tab(Tab::new().pos(710))
                    .line_spacing(LineSpacing::new().line(24 * 15))
                    .align(AlignmentType::Both)
                    .numbering(NumberingId::new(numbering_id), IndentLevel::new(0))
                    .add_run(Run::new().add_text(text))
            );
        }

        document
    }
}
