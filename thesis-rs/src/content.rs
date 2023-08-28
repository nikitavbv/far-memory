pub struct Content {
    pub topic: String,
    pub mentor_name: String,
    pub mentor_title: String,
    pub research_object: String, // об’єкт дослідження
    pub research_subject: String, // предмет дослідження
}

impl Content {
    pub fn new() -> Self {
        Self {
            topic: "Методи та програмні засоби надання програмно-визначеної віддаленої памʼяті у розподілених системах".to_owned(),
            mentor_name: "Павлов Олександр Анатолійович".to_owned(),
            mentor_title: "д.т.н., проф., засл.діяч".to_owned(),
            research_object: "віддалена памʼять у розподілених системах".to_owned(),
            research_subject: "???".to_owned(), // TODO
        }
    }
}