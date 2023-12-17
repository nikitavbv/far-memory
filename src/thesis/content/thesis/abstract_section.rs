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
        paragraph(TextSpan::Multiple(vec![
            TextSpan::Bold(Box::new(TextSpan::Multiple(vec![
                TextSpan::Regular(MultiLanguageString::new("Keywords", "Ключові слова").for_language(language)),
                TextSpan::Regular(": ".to_owned()),
            ]))),
            TextSpan::Multiple(keywords_text),
        ])),
    ])
}
