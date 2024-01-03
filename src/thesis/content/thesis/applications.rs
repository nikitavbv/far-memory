use crate::thesis::engine::{Block, ApplicationBlock};

pub const DEPLOYMENT_DIAGRAM: &str = "deployment_diagram";
pub const COMPONENT_DIAGRAM: &str = "component_diagram";
pub const CLASS_DIAGRAM: &str = "class_diagram";
pub const ACCESS_SEQUENCE: &str = "access_sequence";
pub const BACKGROUND_THREAD_SEQUENCE: &str = "background_thread_sequence";
pub const CODE_LISTING: &str = "code_listing";

pub fn applications() -> Block {
    Block::Multiple(vec![
        Block::Application(ApplicationBlock::new(
            DEPLOYMENT_DIAGRAM,
            "Схема структурна розгортання".to_owned(),
        )), // this should be a deployment diagram - show components and how they communicate
        Block::Application(ApplicationBlock::new(
            COMPONENT_DIAGRAM,
            "Схема структурна компонентів".to_owned(),
        )), // this should be a component diagram - show which data is passed where
        Block::Application(ApplicationBlock::new(
            CLASS_DIAGRAM,
            "Структура бібліотеки клієнта віддаленої памʼяті".to_owned(),
        )), // class diagram for far memory client
        Block::Application(ApplicationBlock::new(
            ACCESS_SEQUENCE,
            "Схема структурна послідовності доступу до обʼєкту що зберігається у віддаленій памʼяті інформаційною системою".to_owned(),
        )), // sequence digram for accessing data for FarMemory<T> end-to-end.
        Block::Application(ApplicationBlock::new(
            BACKGROUND_THREAD_SEQUENCE,
            "Схема структурна послідовності роботи фонового потоку переміщення проміжків".to_owned(),
        )), // sequence diagram for background thread performing swap out and swap in.
        Block::Application(ApplicationBlock::new(
            CODE_LISTING,
            "Лістинг коду".to_owned(),
        ))
    ])
}
