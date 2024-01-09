use crate::thesis::engine::{Block, ApplicationBlock, ApplicationContent};

pub const DEPLOYMENT_DIAGRAM: &str = "deployment_diagram";
pub const COMPONENT_DIAGRAM: &str = "component_diagram";
pub const CLASS_DIAGRAM: &str = "class_diagram";
pub const ACCESS_SEQUENCE: &str = "access_sequence";
pub const BACKGROUND_THREAD_SEQUENCE: &str = "background_thread_sequence";
pub const CODE_LISTING: &str = "code_listing";
pub const PLAGIARISM_REPORT: &str = "plagiarism_report";

pub fn applications() -> Block {
    Block::Multiple(vec![
        Block::Application(ApplicationBlock::new(
            DEPLOYMENT_DIAGRAM,
            "Схема структурна розгортання".to_owned(),
            ApplicationContent::image_from_file("images/deployment_diagram.jpg"),
        )), // this should be a deployment diagram - show components and how they communicate
        Block::Application(ApplicationBlock::new(
            COMPONENT_DIAGRAM,
            "Схема структурна компонентів".to_owned(),
            ApplicationContent::image_from_file("images/component_diagram.jpg"),
        )), // this should be a component diagram - show which data is passed where
        Block::Application(ApplicationBlock::new(
            CLASS_DIAGRAM,
            "Структура бібліотеки клієнта віддаленої памʼяті".to_owned(),
            ApplicationContent::image_from_file("images/class_diagram.jpg"),
        )), // class diagram for far memory client
        Block::Application(ApplicationBlock::new(
            ACCESS_SEQUENCE,
            "Схема структурна послідовності доступу до обʼєкту що зберігається у віддаленій памʼяті інформаційною системою".to_owned(),
            ApplicationContent::image_from_file("images/access_sequence_diagram.jpg"),
        )), // sequence digram for accessing data for FarMemory<T> end-to-end.
        Block::Application(ApplicationBlock::new(
            BACKGROUND_THREAD_SEQUENCE,
            "Схема структурна послідовності роботи фонового потоку переміщення проміжків".to_owned(),
            ApplicationContent::image_from_file("images/background_thread_sequence_diagram.jpg"),
        )), // sequence diagram for background thread performing swap out and swap in.
        code_listing_application(),
        Block::Application(ApplicationBlock::new(
            PLAGIARISM_REPORT,
            "Результати перевірки роботи на співпадіння".to_owned(),
            ApplicationContent::image_from_file("images/plagiarism_report.png"),
        )),
    ])
}

pub fn code_listing_application() -> Block {
    Block::Application(ApplicationBlock::new(
        CODE_LISTING,
        "Лістинг коду".to_owned(),
        ApplicationContent::SourceCode(vec![
            "./Cargo.toml",
            "./src/client/client.rs",
            "./src/client/replacement/mod.rs",
            "./src/client/replacement/replay.rs",
            "./src/client/object.rs",
            "./src/client/vec.rs",
        ]),
    ))
}
