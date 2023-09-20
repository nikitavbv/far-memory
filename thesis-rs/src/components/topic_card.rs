use {
    docx_rs::{
        Docx, 
        Paragraph, 
        Run, 
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

        let text_target = "Планується розробити методи та реалізацію програмного засобу для надання віддаленої памʼяті у розподіленій системі.";

        let text_scientific_novelty = "Удосконалено методи інтеграції віддаленої памʼяті у розподілених системах, підходи до зниження затримки та підвищення відмовстійкості.".to_owned();

        let text_practical_significance = r#"Розроблено програмне забезпечення з відкритим кодом, що надає засоби для розгортання програмно-визначеної віддаленої памʼяті
 у розподілених системах, інтеграції у нове та існуюче програмне забезпечення. Показники швидкодії цього класу памʼяті перевищують значення для файлу підкачки на локальному диску. 
 Реалізація віддаленої памʼяті забезпечує доступ до блоків даних у випадку виходу з ладу вузлів системи."#;

        self
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
            .add_paragraph(Paragraph::new().add_tab(Tab::new().pos(710)).line_spacing(LineSpacing::new().before(100)).align(AlignmentType::Both).add_run(Run::new().add_tab().add_text(&content.aim_of_study_short_ua)))
            .add_paragraph(Paragraph::new().add_tab(Tab::new().pos(710)).add_run(Run::new().add_tab().add_text("Об'єкт дослідження: ").add_text(content.research_object.for_language(&Language::Ukrainian)).add_text(".")))
            .add_paragraph(Paragraph::new().add_tab(Tab::new().pos(710)).add_run(Run::new().add_tab().add_text("Предмет дослідження: ").add_text(content.research_subject.for_language(&Language::Ukrainian)).add_text(".")))
            .add_paragraph(Paragraph::new()
                .line_spacing(LineSpacing::new().before(300))
                .add_run(Run::new().add_text("Задачі, що вирішуються в роботі").size(2 * 14).color("#434343")))
            .add_paragraph(Paragraph::new().line_spacing(LineSpacing::new().before(100)).add_run(Run::new().add_text(text_target)))
            .add_paragraph(Paragraph::new().add_run(Run::new().add_text("Задачі, що вирішуються:")))
            .add_unordered_list_component(context, content.tasks.clone())
            .add_paragraph(Paragraph::new()
                .line_spacing(LineSpacing::new().before(300))
                .add_run(Run::new().add_text("Наукова новизна").size(2 * 14).color("#434343")))
            .add_paragraph(Paragraph::new().line_spacing(LineSpacing::new().before(100)).add_run(Run::new().add_text(text_scientific_novelty)))
            .add_paragraph(Paragraph::new()
                .line_spacing(LineSpacing::new().before(300))
                .add_run(Run::new().add_text("Практичне значення").size(2 * 14).color("#434343")))
            .add_paragraph(Paragraph::new().line_spacing(LineSpacing::new().before(100)).align(AlignmentType::Both).add_run(Run::new().add_text(text_practical_significance)))
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