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
        content::{Content, Language},
        context::Context,
    },
};

pub trait TopicCardDocument {
    fn add_topic_card_document(self, context: &mut Context, content: &Content) -> Self;
}

impl TopicCardDocument for Docx {
    fn add_topic_card_document(self, context: &mut Context, content: &Content) -> Self {
        let text_problem = r#"Середній рівень використання оперативної памʼяті у сучасних великих центрах обробки даних (датацентрах) становить приблизно 60%. 
Оператори центрів обробки даних зацікавлені у ефективному використанні ресурсів, тому що це дозволяє використовувати менше фізичних вузлів (серверного обладнання) для розгортання програмного забезпечення 
та знизити витрати на сервери. Одним зі способів збільшення ефективності використання оперативної памʼяті є підхід що називається Far Memory (віддалена памʼять).
        "#;
        let text_problem_2 = r#"Суть цього методу полягає в тому, що 
програми, які потребують значних обʼємів памʼяті (сховища даних, та сервіси щовиконують обробку даних у цих сховищах) можуть передавати деякі блоки памʼяті на зберігання у памʼять інших серверів, які мають низький 
рівень використання оперативної памʼяті. У порівнянні з використанням файлу підкачки (swap file), перевага віддаленої памʼяті полягає у більш низькому рівні затримки (latency) та більш високій відмовостійкості. 
        "#;
        let text_problem_3 = "Існуючі реалізації Far Memory є або пропрієтарними (та недоступними для використання ззовні компаній що іх розробили - наприклад Google) або не мають необхідного функціоналу для використання на практиці.";

        let text_goal = r#"Розробити архітектуру програмного засобу та її відкриту реалізацію, яка надає віддалену памʼять у розподіленій системі з багатьох вузлів, є простою у розгортанні та інтеграції у нове та існуюче програмне забезпечення.
Архітектура реалізації віддаленої памʼяті повинна передбачати відмовостійкість (дані не втрачаються при виході з ладу вузлів) та достатній рівень швидкодії (вищий за показник для файлу підкачки на локальному диску)."#;

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
            .add_paragraph(Paragraph::new().add_tab(Tab::new().pos(710)).line_spacing(LineSpacing::new().before(100)).align(AlignmentType::Both).add_run(Run::new().add_tab().add_text(text_problem)))
            .add_paragraph(Paragraph::new().add_tab(Tab::new().pos(710)).align(AlignmentType::Both).add_run(Run::new().add_tab().add_text(text_problem_2)))
            .add_paragraph(Paragraph::new().add_tab(Tab::new().pos(710)).align(AlignmentType::Both).add_run(Run::new().add_tab().add_text(text_problem_3)))
            .add_paragraph(Paragraph::new()
                .line_spacing(LineSpacing::new().before(300))
                .add_run(Run::new().add_text("Мета").size(2 * 14).color("#434343")))
            .add_paragraph(Paragraph::new().line_spacing(LineSpacing::new().before(100)).add_run(Run::new().add_text(text_goal)))
            .add_paragraph(Paragraph::new().add_run(Run::new().add_text("Об'єкт дослідження: ").add_text(content.research_object.for_language(&Language::Ukrainian)).add_text(".")))
            .add_paragraph(Paragraph::new().add_run(Run::new().add_text("Предмет дослідження: ").add_text(content.research_subject.for_language(&Language::Ukrainian)).add_text(".")))
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