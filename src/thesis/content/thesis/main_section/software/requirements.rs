use crate::thesis::engine::{Block, paragraph, section_header, subsection_header};

pub fn requirements() -> Block {
    Block::Multiple(vec![
        section_header("Опис програмного забезпечення"),
        subsection_header("Вимоги до програмного забезпечення"),
        paragraph("Як було зазначено в попередніх розділах, до реалізації віддаленої памʼяті, що розглядається в цій роботі, висуваються \
вимоги які визначені розглянутими особливостями середовища та програмного забезпечення, у яке вона інтегрується."),
        paragraph("Віддалена памʼять повинна інтегруватися у програмне забезпечення за допомогою бібліотеки або віртуального блокового \
пристрою. У разі використання бібліотеки, розробнику прикладного програмного забезпечення повинен надаватись клієнт віддаленої памʼяті, \
який надає засоби для зберігання послідовностей байт, обʼєктів наданих користувачем та структур даних оптимізованих для роботи з віддаленою \
памʼяттю."),
        paragraph("Реалізація віддаленої памʼяті повинна коректно обробляти події виходу з ладу віддалених вузлів зберігання та підтримувати \
запланований вивід вузлів на обслуговування, що є типовою вимогою для програмного забезпечення що працює у розподіленій системі. У разі \
виходу вузла зберігання з ладу, клієнт віддаленої памʼяті повинен мінімізувати вирогідність втрати даних через їх відновлення з памʼяті \
інших вузлів."),
        paragraph("Так як рівень швидкодії віддаленої памʼяі напряму впливає на доцільність її використання для різних типів програмного \
забезпечення, то має сенс визначити вимоги щодо часу доступу до даних у віддаленій памʼяті. Мінімальним рівнем що робить використання \
віддаленої памʼяті виправданим є той рівень, де час доступу до даних у віддаленій памʼяті є меншим ніж час доступу до даних такого самого \
розміру розміщених на дисковому сховищі у сучасній інфраструктурі з дізагрегованими ресурсами. Якщо віддалена памʼять буде більш повільною \
у порівнянні, то її використання не є доцільним, так як програмне забезпечення буде працювати швидше у разі зберігання даних на диску."),
        paragraph("До апаратної платформи, на якій розгортаються вузли зберігання, обчислення та керування висуваються наступні вимоги:"),
        Block::UnorderedList(vec![
            "процесор архітектури x86 або ARM з тактовою частотою не менше 1 ГГц, з одним ядром чи більше".to_owned(),
            "оперативна памʼять обʼємом не менше 1 Гб".to_owned(),
            "вільний дисковий простір обʼємом не менше 10Гб (тільки для вузла керування)".to_owned(),
            "всі вузли мають доступ до всіх інших вузлів по мережі, можуть відкрити зʼєднання та передавати та отримувати дані".to_owned(),
            "пропускна здатність мережі не менше 100Мбіт/сек, затримка між вузлами - не більше 10 мілісекунд".to_owned(),
        ]),
        paragraph("Усі вузли розгортаються на операційній системі Linux."),
    ])
}
