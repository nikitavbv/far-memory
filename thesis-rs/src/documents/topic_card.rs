use {
    docx_rs::{
        Docx, 
        Paragraph, 
        Run, 
        BreakType, 
        Table, 
        TableRow, 
        TableCell, 
        WidthType, 
        LineSpacing,
        AbstractNumbering,
        Numbering,
        Level,
        Start,
        Tab,
        NumberFormat,
        LevelText,
        LevelJc,
        SpecialIndentType,
        AlignmentType,
        NumberingId,
        IndentLevel,
    },
    crate::{
        content::Content,
        context::Context,
    },
};

pub trait TopicCardDocument {
    fn add_topic_card_document(self, context: &mut Context, content: &Content) -> Self;
}

impl TopicCardDocument for Docx {
    fn add_topic_card_document(self, context: &mut Context, content: &Content) -> Self {
        let text_problem = r#"
Середній рівень використання оперативної памʼяті у сучасних великих центрах обробки даних (датацентрах) становить приблизно 60%. 
Оператори центрів обобки даних зацікавлені у ефективному використанні ресурсів, тому що це дозволяє використовувати менше фізичних вузлів (серверного обладнання) для розгортання програмного забезпечення 
та знизити витрати на нього.
        "#;

        self
            .add_paragraph(Paragraph::new().add_run(Run::new().add_break(BreakType::TextWrapping)))
            .add_table(Table::new(vec![
                TableRow::new(vec![
                    TableCell::new()
                        .add_paragraph(Paragraph::new().line_spacing(LineSpacing::new().after(100).before(60)).add_run(Run::new().size(2 * 10).add_text("ІП-22мп")))
                        .width(1500, WidthType::Dxa),
                    TableCell::new()
                        .add_paragraph(Paragraph::new().line_spacing(LineSpacing::new().after(100).before(60)).add_run(Run::new().size(2 * 10).add_text("Волобуєв Нікіта")))
                        .width(6000, WidthType::Dxa),
                    TableCell::new()
                        .add_paragraph(Paragraph::new().line_spacing(LineSpacing::new().after(100).before(60)).add_run(Run::new().size(2 * 10).add_text(content.mentor.full_name())))
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
            .add_paragraph(Paragraph::new().line_spacing(LineSpacing::new().before(100)).align(AlignmentType::Both).add_run(Run::new().add_text(text_problem)))
            .add_paragraph(Paragraph::new()
                .line_spacing(LineSpacing::new().before(300))
                .add_run(Run::new().add_text("Мета").size(2 * 14).color("#434343")))
            .add_paragraph(Paragraph::new().line_spacing(LineSpacing::new().before(100)).add_run(Run::new().add_text("Підвищити швидкість роботи, якість або надійність (вказати критерії) / розширити рамки використання / забезпечити певну властивість ПЗ, тощо.")))
            .add_paragraph(Paragraph::new().add_run(Run::new().add_text("Об'єкт дослідження: програмне забезпечення для")))
            .add_paragraph(Paragraph::new().add_run(Run::new().add_text("Предмет дослідження: процеси розроблення, модифікації, аналізу, забезпечення якості, впровадження і супроводження програмного забезпечення")))
            .add_paragraph(Paragraph::new()
                .line_spacing(LineSpacing::new().before(300))
                .add_run(Run::new().add_text("Задачі, що вирішуються в роботі").size(2 * 14).color("#434343")))
            .add_paragraph(Paragraph::new().line_spacing(LineSpacing::new().before(100)).add_run(Run::new().add_text("Планується глобальна задача.")))
            .add_paragraph(Paragraph::new().add_run(Run::new().add_text("Задачі, що вирішуються:")))
            .add_unordered_list_component(context, vec![
                "аналіз існуючих рішень".to_owned(),
                "".to_owned(),
                "".to_owned(),
                "оцінка ефективності запропонованого рішення".to_owned(),
            ])
            .add_paragraph(Paragraph::new()
                .line_spacing(LineSpacing::new().before(300))
                .add_run(Run::new().add_text("Наукова новизна").size(2 * 14).color("#434343")))
            .add_paragraph(Paragraph::new().line_spacing(LineSpacing::new().before(100)).add_run(Run::new().add_text("Удосконалено / Набуло подальшого розвитку.")))
            .add_paragraph(Paragraph::new().add_run(Run::new().add_text("(формально: фраза з “Рекомендацій науковим керівникам магістрів щодо вибору тем”, наприклад:  нові або вдосконалені методи, способи, підходи, що підвищують ефективність розроблення ПЗ або його модифікації)")))
            .add_paragraph(Paragraph::new()
                .line_spacing(LineSpacing::new().before(300))
                .add_run(Run::new().add_text("Практичне значення").size(2 * 14).color("#434343")))
            .add_paragraph(Paragraph::new().line_spacing(LineSpacing::new().before(100)).add_run(Run::new().add_text("Запропоновано / Розроблено таке-то програмне забезпечення або архітектура, тощо")))
    }
}

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
                        LevelText::new("• "),
                        LevelJc::new("left")
                    ).indent(None, Some(SpecialIndentType::FirstLine(425)), None, None))
            )
            .add_numbering(Numbering::new(numbering_id, numbering_id));
        
        for i in 0..list.len() {
            document = document.add_paragraph(
                Paragraph::new()
                    .add_tab(Tab::new().pos(710))
                    .line_spacing(LineSpacing::new().line(24 * 15))
                    .align(AlignmentType::Both)
                    .numbering(NumberingId::new(numbering_id), IndentLevel::new(0))
                    .add_run(Run::new().add_text(list.get(i).unwrap()).add_text(if i == list.len() - 1 { "." } else { ";" }))
            ); 
        }

        document
    }
}