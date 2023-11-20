use crate::thesis::engine::{Block, section_header, subsection_header, paragraph, SubsectionHeaderBlock};

mod components;
mod integration;
mod performance;
mod reliability;

pub fn far_memory_method() -> Block {
    Block::Multiple(vec![
        section_header("Методи та засоби надання віддаленої памʼяті"),
        paragraph("Загалом, принцип роботи віддаленої памʼяті полягає в тому, що програмне забезпечення передає у віддалену памʼять дані для зберігання, а коли до цих даних потрібен доступ, то \
реалізація віддаленої памʼяті за запитом від застосунку переміщує дані з інших більш повільних пристроїв зберігання даних у локальну оперативну памʼять. Після переміщення, програмне \
забезпечення працює з даними так само, як і з будь-якими іншими даними у оперативній памʼяті. Після того, як робота з даними закінчена, вони знов переміщуються на зберігання у інший клас памʼяті. \
Саме це надає можливість зменшити використання оперативної памʼяті вузла та обробляти більше даних ніж обʼєм памʼяті прозоро для програмного забезпечення (тобто без значних змін у код, та те, як \
застосунок працює з даними)."),

        components::components(),
        integration::integration(),
        reliability::reliability(),
        performance::performance(),

        Block::SubsectionHeader(SubsectionHeaderBlock::without_numbering("Висновки до розділу".to_owned())),
        paragraph("В даному розділі було розглянуто проблему надання віддаленої памʼяті та метод..."),

        // in third section explain that a demo app was implemented to measure how well everything works and the hardware of the test environment.
    ])
}
