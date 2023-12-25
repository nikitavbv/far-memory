use crate::thesis::engine::{Block, section_header, paragraph, SubsectionHeaderBlock, subsection_header};

pub fn marketing() -> Block {
    Block::Multiple(vec![
        section_header("Маркетинговий аналіз стартап-проекту"),

        subsection_header("Опис ідеї проекту"),
        paragraph("Для опису ідеї проекту проаналізуємо зміст ідеї що пропонується, можливі напрямки застосування, та основні вигоди що може отримати \
користувач цього програмного продукту. Результати аналізу наведено у наступній таблиці."),
        /*Block::Table {
            columns: vec![
                "Зміст ідеї".to_owned(),
                "Напрямки застосування".to_owned(),
                "Вигоди для користувача".to_owned(),
            ],
            rows: vec![
                // TODO: finish this table
            ],
        },*/

        // TODO: finish this section

        Block::SubsectionHeader(SubsectionHeaderBlock::without_numbering("Висновки до розділу".to_owned())),
        Block::Placeholder(Box::new(paragraph("some text here")), "add some text here".to_owned()),
    ])
}
