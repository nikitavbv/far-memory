use crate::thesis::{engine::{Block, paragraph, TextSpan}, content::{Language, MultiLanguageString, keywords}};

pub fn abstract_section(language: &Language) -> Block {
    let mut keywords_text = vec![];
    let content_keywords = keywords();
    for i in 0..content_keywords.len() {
        let keyword = content_keywords.get(i).unwrap();

        keywords_text.push(TextSpan::Regular(keyword.for_language(language).to_uppercase()));
        if i < content_keywords.len() - 1 {
            keywords_text.push(TextSpan::Regular(", ".to_owned()));
        }
    }

    Block::Multiple(vec![
        // TODO: convert publications from `abstract_section` component to blocks.
        // Block::OrderedList(vec!["Ясенова А.В. Застосування алгоритмів кластеризації на ринку іноземних валют/ А.В.Ясенова, О.А. Халус // Матеріали V всеукраїнської науковопрактичної конференції молодих вчених та студентів «Інформаційні системи та технології управління» (ІСТУ-2020) – м. Київ: НТУУ «КПІ ім. Ігоря Сікорського», 26-27 листопада 2020 р.".into()]),
        paragraph(TextSpan::Multiple(vec![
            TextSpan::Bold(Box::new(TextSpan::Multiple(vec![
                TextSpan::Regular(MultiLanguageString::new("Keywords", "Ключові слова").for_language(language)),
                TextSpan::Regular(": ".to_owned()),
            ]))),
            TextSpan::Multiple(keywords_text),
        ])),
    ])
}
