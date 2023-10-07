pub use self::{
    thesis::{thesis_content, thesis_docx_template},
    topic_card::topic_card_docx_template,
};

pub mod documentation;
pub mod thesis;
pub mod topic_card;

pub struct Content {
    pub topic: String,
    pub mentor: Person,
    pub mentor_title: String,
    pub aim_of_study_short_ua: String,
    pub aim_of_study: MultiLanguageString, // Мета
    pub research_object: MultiLanguageString, // об’єкт дослідження
    pub research_subject: MultiLanguageString, // предмет дослідження
    pub keywords: Vec<MultiLanguageString>,
    pub tasks: Vec<MultiLanguageString>,
}

impl Content {
    pub fn new() -> Self {
        let aim_of_study_short_ua = "Розробити архітектуру програмного засобу та її відкриту реалізацію, яка надає віддалену памʼять у розподіленій системі з багатьох вузлів, є простою у розгортанні та інтеграції у нове та існуюче програмне забезпечення. Архітектура реалізації віддаленої памʼяті повинна передбачати відмовостійкість (дані не втрачаються при виході з ладу вузлів) та достатній рівень швидкодії (вищий за показник для файлу підкачки на локальному диску).".to_owned();
        let aim_of_study = MultiLanguageString::new(
            "The main target is to develop a software architecture and its open implementation that provides far memory in a multi-node distributed system, is easy to deploy and integrate into new and existing software. The architecture of the implementation of remote memory should provide fault tolerance (no data is lost when nodes fail) and a sufficient level of performance (higher than the performance of swap file on the local disk).",
            format!(
                "Основною метою є {}{}", 
                aim_of_study_short_ua.chars().nth(0).unwrap().to_lowercase(), 
                aim_of_study_short_ua.chars().skip(1).collect::<String>()
            ),
        );

        Self {
            topic: "Методи та програмні засоби надання програмно-визначеної віддаленої памʼяті у розподілених системах".to_owned(),
            mentor: Person::new("Павлов".to_owned(), "Олександр".to_owned(), "Анатолійович".to_owned()),
            mentor_title: "д.т.н., проф., засл.діяч".to_owned(),
            aim_of_study_short_ua,
            aim_of_study,
            research_object: MultiLanguageString::new("far memory in distributed systems", "віддалена памʼять у розподілених системах"),
            research_subject: MultiLanguageString::new(
                "methods of ensuring fast access to data blocks in far memory, their replication, far memory deployment and integration into software", 
                "методи забезпечення швидкого доступу до блоків даних у віддаленій памʼяті, їх реплікація, розгортання та інтеграція віддаленої памʼяті у програмне забезпечення"
            ),
            keywords: vec![
                MultiLanguageString::new(
                    "Far memory",
                    "Віддалена памʼять"
                ),
                MultiLanguageString::new(
                    "Distributed systems",
                    "Розподілені системи"
                ),
                MultiLanguageString::new(
                    "Computer networks",
                    "Комп'ютерні мережі"
                ),
                MultiLanguageString::new(
                    "Data structures",
                    "Структури даних"
                ),
                MultiLanguageString::new(
                    "Computer networks",
                    "Комп'ютерні мережі"
                ),
                MultiLanguageString::new(
                    "Linux",
                    "Linux"
                ),
            ],
            tasks: vec![
                MultiLanguageString::new(
                    "perform analysis of existing far memory implementations and methods".to_owned(), 
                    "провести аналіз існуючих реалізацій та методів надання віддаленої памʼяті".to_owned(),
                ),
                MultiLanguageString::new(
                    "develop far memory integration methods into new and existing software".to_owned(), 
                    "розробити методи інтеграції віддаленої памʼяті у нове та існуюче програмне забезпечення".to_owned(),
                ),
                MultiLanguageString::new(
                    "develop architecture, structure and interaction between far memory components".to_owned(), 
                    "розробити архітектуру, структуру та взаємодію компонентів віддаленої памʼяті".to_owned(),
                ),
                MultiLanguageString::new(
                    "optimize far memory spans access latency".to_owned(), 
                    "оптимізувати затримку доступу до блоків у віддаленій памʼяті".to_owned(),
                ),
                MultiLanguageString::new(
                    "develop methods to ensure far memory resiliency".to_owned(), 
                    "розробити методи забезпечення відмовостійкості віддаленої памʼяті".to_owned(),
                ),
                MultiLanguageString::new(
                    "assess solution efficiency".to_owned(), 
                    "провести оцінку ефективності запропонованого рішення".to_owned(),
                ),
            ],
        }
    }
}

pub struct Person {
    pub first_name: String,
    pub last_name: String,
    pub surname: String,
}

impl Person {
    pub fn new(first_name: String, last_name: String, surname: String) -> Self {
        Self {
            first_name,
            last_name,
            surname,
        }
    }

    pub fn full_name(&self) -> String {
        format!("{} {} {}", self.first_name, self.last_name, self.surname)
    }
}

#[derive(Debug, Clone)]
pub enum Language {
    English,
    Ukrainian,
}

pub struct MultiLanguageString {
    english: String,
    ukrainian: String,
}

impl MultiLanguageString {
    pub fn new(english: impl Into<String>, ukrainian: impl Into<String>) -> Self {
        Self {
            english: english.into(),
            ukrainian: ukrainian.into(),
        }
    }

    pub fn for_language(&self, language: &Language) -> String {
        match language {
            &Language::English => &self.english,
            &Language::Ukrainian => &self.ukrainian,
        }.to_owned()
    }
}

#[derive(Debug, Clone)]
pub struct AbstractContent {
    pub total_pages: u32,
    pub total_images: u32,
    pub total_tables: u32,
    pub total_applications: u32,
    pub total_references: u32,
}

pub struct EnglishNumeralString {
    word: String,
}

impl EnglishNumeralString {
    pub fn new(word: String) -> Self {
        Self {
            word,
        }
    }

    pub fn for_value(&self, value: u32) -> String {
        if value == 1 {
            self.word.clone()
        } else {
            format!("{}s", self.word)
        }
    }
}

pub struct UkrainianNumeralString {
    word: String, // зображення
    word_multi_gen: String, // зображень
    word_gen: String, // зображеня
}

impl UkrainianNumeralString {
    pub fn new(word: String, word_multi_gen: String, word_gen: String) -> Self {
        Self {
            word,
            word_multi_gen,
            word_gen,
        }
    }

    pub fn for_value(&self, value: u32) -> String {
        if [0, 5, 6, 7, 8, 9].contains(&(value % 10)) || (10..=20).contains(&value) {
            self.word_gen.clone()
        } else if [2, 3, 4].contains(&(value % 10)) {
            self.word_multi_gen.clone()
        } else {
            self.word.clone()
        }
    }
}

pub struct MultiLanguageNumeralString {
    english: EnglishNumeralString,
    ukrainian: UkrainianNumeralString,
}

impl MultiLanguageNumeralString {
    pub fn new(english: EnglishNumeralString, ukrainian: UkrainianNumeralString) -> Self {
        Self {
            english,
            ukrainian,
        }
    }

    pub fn for_language_and_value(&self, language: &Language, value: u32) -> String {
        match language {
            Language::English => self.english.for_value(value),
            Language::Ukrainian => self.ukrainian.for_value(value),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn numeral_ukrainian_simple() {
        let numeral = UkrainianNumeralString::new("ілюстрація".to_owned(), "ілюстрації".to_owned(), "ілюстрацій".to_owned());
        assert_eq!(numeral.for_value(1), "ілюстрація");
        assert_eq!(numeral.for_value(2), "ілюстрації");
        assert_eq!(numeral.for_value(3), "ілюстрації");
        assert_eq!(numeral.for_value(4), "ілюстрації");
        assert_eq!(numeral.for_value(5), "ілюстрацій");
        assert_eq!(numeral.for_value(6), "ілюстрацій");
        assert_eq!(numeral.for_value(7), "ілюстрацій");
        assert_eq!(numeral.for_value(8), "ілюстрацій");
        assert_eq!(numeral.for_value(9), "ілюстрацій");
        assert_eq!(numeral.for_value(10), "ілюстрацій");
        assert_eq!(numeral.for_value(11), "ілюстрацій");
        assert_eq!(numeral.for_value(12), "ілюстрацій");
        assert_eq!(numeral.for_value(13), "ілюстрацій");
        assert_eq!(numeral.for_value(14), "ілюстрацій");
        assert_eq!(numeral.for_value(15), "ілюстрацій");
        assert_eq!(numeral.for_value(16), "ілюстрацій");
        assert_eq!(numeral.for_value(17), "ілюстрацій");
        assert_eq!(numeral.for_value(18), "ілюстрацій");
        assert_eq!(numeral.for_value(19), "ілюстрацій");
        assert_eq!(numeral.for_value(20), "ілюстрацій");
        assert_eq!(numeral.for_value(21), "ілюстрація");
        assert_eq!(numeral.for_value(22), "ілюстрації");
        assert_eq!(numeral.for_value(23), "ілюстрації");
        assert_eq!(numeral.for_value(24), "ілюстрації");
        assert_eq!(numeral.for_value(25), "ілюстрацій");
        assert_eq!(numeral.for_value(26), "ілюстрацій");
        assert_eq!(numeral.for_value(27), "ілюстрацій");
        assert_eq!(numeral.for_value(28), "ілюстрацій");
        assert_eq!(numeral.for_value(29), "ілюстрацій");
        assert_eq!(numeral.for_value(30), "ілюстрацій");
        assert_eq!(numeral.for_value(31), "ілюстрація");
    }
}