pub struct Content {
    pub topic: String,
    pub mentor: Person,
    pub mentor_title: String,
    pub research_object: MultiLanguageString, // об’єкт дослідження
    pub research_subject: MultiLanguageString, // предмет дослідження
}

impl Content {
    pub fn new() -> Self {
        Self {
            topic: "Методи та програмні засоби надання програмно-визначеної віддаленої памʼяті у розподілених системах".to_owned(),
            mentor: Person::new("Павлов".to_owned(), "Олександр".to_owned(), "Анатолійович".to_owned()),
            mentor_title: "д.т.н., проф., засл.діяч".to_owned(),
            research_object: MultiLanguageString::new("far memory in distributed systems", "віддалена памʼять у розподілених системах"),
            research_subject: MultiLanguageString::new("???", "???"), // TODO
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
