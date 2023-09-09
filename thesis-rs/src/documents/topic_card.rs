use {
    docx_rs::{Docx, Paragraph, Run, BreakType, Table, TableRow, TableCell, WidthType, LineSpacing},
    crate::content::Content,
};

pub trait TopicCardDocument {
    fn add_topic_card_document(self, content: &Content) -> Self;
}

impl TopicCardDocument for Docx {
    fn add_topic_card_document(self, content: &Content) -> Self {
        self
            .add_paragraph(Paragraph::new().add_run(Run::new().add_break(BreakType::TextWrapping)))
            .add_table(Table::new(vec![
                TableRow::new(vec![
                    TableCell::new()
                        .add_paragraph(Paragraph::new().line_spacing(LineSpacing::new().after(100).before(60)).add_run(Run::new().size(2 * 10).add_text("група")))
                        .width(1500, WidthType::Dxa),
                    TableCell::new()
                        .add_paragraph(Paragraph::new().line_spacing(LineSpacing::new().after(100).before(60)).add_run(Run::new().size(2 * 10).add_text("студент")))
                        .width(6000, WidthType::Dxa),
                    TableCell::new()
                        .add_paragraph(Paragraph::new().line_spacing(LineSpacing::new().after(100).before(60)).add_run(Run::new().size(2 * 10).add_text("керівник")))
                        .width(4000, WidthType::Dxa),
                ]),
            ]))
            .add_paragraph(Paragraph::new()
                .line_spacing(LineSpacing::new().before(300))
                .add_run(Run::new().add_text("Тема").size(2 * 14).color("#434343")))
            .add_paragraph(Paragraph::new().line_spacing(LineSpacing::new().before(100)).add_run(Run::new().bold().size(2 * 11).add_text(&content.topic)))
            .add_paragraph(Paragraph::new()
                .line_spacing(LineSpacing::new().before(300))
                .add_run(Run::new().add_text("Проблема, що розглядається").size(2 * 14).color("#434343")))
            .add_paragraph(Paragraph::new().line_spacing(LineSpacing::new().before(100)).add_run(Run::new().add_text("Один абзац про проблему.")))
            .add_paragraph(Paragraph::new()
                .line_spacing(LineSpacing::new().before(300))
                .add_run(Run::new().add_text("Мета").size(2 * 14).color("#434343")))
            .add_paragraph(Paragraph::new().line_spacing(LineSpacing::new().before(100)).add_run(Run::new().add_text("Підвищити швидкість роботи, якість або надійність (вказати критерії) / розширити рамки використання / забезпечити певну властивість ПЗ, тощо.")))
            .add_paragraph(Paragraph::new().add_run(Run::new().add_text("Об'єкт дослідження: програмне забезпечення для")))
            .add_paragraph(Paragraph::new().add_run(Run::new().add_text("Предмет дослідження: процеси розроблення, модифікації, аналізу, забезпечення якості, впровадження і супроводження програмного забезпечення")))
            .add_paragraph(Paragraph::new()
                .line_spacing(LineSpacing::new().before(300))
                .add_run(Run::new().add_text("Задачі, що вирішуються в роботі").size(2 * 14).color("#434343")))
    }
}