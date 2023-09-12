pub struct Content {
    pub topic: String,
    pub mentor: Person,
    pub mentor_title: String,
    pub aim_of_study_short_ua: String,
    pub aim_of_study: MultiLanguageString, // Мета
    pub research_object: MultiLanguageString, // об’єкт дослідження
    pub research_subject: MultiLanguageString, // предмет дослідження
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
                "methods for ensuring fast access to data blocks in far memory, their replication, far memory deployment and integration into software", 
                "методи забезпечення швидкого доступу до блоків даних у віддаленій памʼяті, їх реплікація, розгортання та інтеграція віддаленої памʼяті у програмне забезпечення"
            ),
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
