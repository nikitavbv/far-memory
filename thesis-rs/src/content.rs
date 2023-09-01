pub struct Content {
    pub topic: String,
    pub mentor: Person,
    pub mentor_title: String,
    pub research_object: String, // об’єкт дослідження
    pub research_subject: String, // предмет дослідження
}

impl Content {
    pub fn new() -> Self {
        Self {
            topic: "Методи та програмні засоби надання програмно-визначеної віддаленої памʼяті у розподілених системах".to_owned(),
            mentor: Person::new("Павлов".to_owned(), "Олександр".to_owned(), "Анатолійович".to_owned()),
            mentor_title: "д.т.н., проф., засл.діяч".to_owned(),
            research_object: "віддалена памʼять у розподілених системах".to_owned(),
            research_subject: "???".to_owned(), // TODO
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