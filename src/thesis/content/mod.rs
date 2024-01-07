pub use self::{
    thesis::{thesis_content, thesis_content_for_plagiarism_check, thesis_application_code_listing, thesis_docx_template, practice_report_content},
    topic_card::topic_card_docx_template,
};

pub mod conference_abstract;
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
    pub scientific_novelty: MultiLanguageString,
    pub keywords: Vec<MultiLanguageString>,
    pub tasks: Vec<MultiLanguageString>,
}

impl Content {
    pub fn new() -> Self {
        let aim_of_study_short_ua = "Підвищення ефективності використання оперативної памʼяті за рахунок розробки ефективного методу надання віддаленої памʼяті \
в інформаційному забезпеченні сучасних центрів обробки даних.".to_owned();
        let aim_of_study = MultiLanguageString::new(
            "Improving the efficiency of using far memory by developing an effective method for providing far memory to the information software of modern \
datacenters.".to_owned(),
            aim_of_study_short_ua.clone()
        );

        Self {
            topic: "Методи та програмні засоби надання програмно-визначеної віддаленої памʼяті у розподілених системах".to_owned(),
            mentor: Person::new("Павлов".to_owned(), "Олександр".to_owned(), "Анатолійович".to_owned()),
            mentor_title: "д.т.н., проф., засл.діяч".to_owned(),
            aim_of_study_short_ua,
            aim_of_study,
            research_object: MultiLanguageString::new("far memory in distributed information systems", "віддалена памʼять у розподілених інформаційних системах"),
            research_subject: MultiLanguageString::new(
                "the process of creation of the architecture of software tools that implement methods for providing fast access to span in far memory, their replication, \
deployment and integration of far memory into the software",
                "процес створення архітектури програмних засобів що реалізують методи забезпечення швидкого доступу до блоків даних у віддаленій памʼяті, їх \
реплікація, розгортання та інтеграція віддаленої памʼяті у програмне забезпечення"
            ),
            scientific_novelty: MultiLanguageString::new(
                "unlike existing methods, span replacement problem is solved statistically more efficiently by implementing the adaptation of span access prediction model parameters \
based on statistics that are continously collected during runtime of software.".to_owned(),
                "на відміну від існуючих методів, задача заміщення проміжків вирішена статистично більш ефективно за рахунок реалізації адаптації параметрів моделі \
прогнозування доступу на основі використання статистики, що неперервно збирається в процесі роботи програмного забезпечення.".to_owned(),
            ),
            keywords: keywords(),
            tasks: thesis_tasks(),
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

pub fn thesis_tasks() -> Vec<MultiLanguageString> {
    vec![
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
            "decrease average latency of far memory spans access by using span replacement algorithm that relies on memory access statistics and predictive models".to_owned(),
            "знизити в середьному затримку доступу до блоків у віддаленій памʼяті за рахунок використання алгоритму заміщення, що спирається на статистику доступу \
до памʼяті та використання прогнозних моделей".to_owned(),
        ),
        MultiLanguageString::new(
            "develop methods to ensure far memory resiliency".to_owned(),
            "розробити методи забезпечення відмовостійкості віддаленої памʼяті".to_owned(),
        ),
        MultiLanguageString::new(
            "assess solution efficiency".to_owned(),
            "провести оцінку ефективності запропонованого рішення".to_owned(),
        ),
    ]
}

pub fn classification_code() -> String {
    "004.75".to_owned()
}

pub fn keywords() -> Vec<MultiLanguageString> {
    vec![
        MultiLanguageString::new(
            "far memory",
            "віддалена памʼять"
        ),
        MultiLanguageString::new(
            "distributed systems",
            "розподілені системи"
        ),
        MultiLanguageString::new(
            "computer networks",
            "комп'ютерні мережі"
        ),
        MultiLanguageString::new(
            "page replacement",
            "заміщення сторінок"
        ),
        MultiLanguageString::new(
            "Linux",
            "Linux"
        ),
        MultiLanguageString::new(
            "Rust",
            "Rust"
        )
    ]
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
