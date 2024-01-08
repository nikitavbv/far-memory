use crate::thesis::engine::{Block, section_header, subsection_header, paragraph, SubsectionHeaderBlock};

mod components;
mod integration;
mod performance;
mod reliability;

pub fn far_memory_method() -> Block {
    Block::Multiple(vec![
        section_header("Методи та засоби надання віддаленої памʼяті"),
        paragraph("Загалом, принцип роботи віддаленої памʼяті полягає в тому, що програмне забезпечення передає у віддалену памʼять дані для зберігання, а коли до цих даних потрібен доступ, то \
реалізація віддаленої памʼяті за запитом від застосунку переміщує дані з інших більш повільних пристроїв зберігання даних у локальну оперативну памʼять. Після переміщення, програмне \
забезпечення працює з даними так само, як і з будь-якими іншими даними у оперативній памʼяті. Після того, як робота з даними закінчена, вони знов переміщуються на зберігання у інший клас памʼяті. \
Саме це надає можливість зменшити використання оперативної памʼяті вузла та обробляти більше даних ніж обʼєм памʼяті прозоро для програмного забезпечення (тобто без значних змін у код, та те, як \
застосунок працює з даними)."),

        components::components(),
        integration::integration(),
        reliability::reliability(),
        performance::performance(),

        Block::SubsectionHeader(SubsectionHeaderBlock::without_numbering("Висновки до розділу".to_owned())),
        paragraph("В даному розділі було розглянуто проблему надання віддаленої памʼяті у розподілених інформаційних системах. Описано середовище та принцип роботи віддаленої памʼяті у \
ньому."),
        paragraph("З урахуванням специфіки середовищ, для інтеграції в які призначена ця реалізація віддаленої памʼяті, визначено компоненти системи. Це вузли обчислення, \
вузли зберігання та вузел керування. Визначено які дані та як передаються між цими вузлами, як виконується збір статистики про роботу кластеру та керування ним."),
        paragraph("З формулювання проблеми що вирішується, середовища та структури компонентів випливають підзадачі, які необхідно розглянути для стоврення програмного \
засобу надання віддаленої памʼяті у розподіленій системі."),
        paragraph("Розглянуто проблему інтеграції віддаленої памʼяті у програмне забезпечення. Запропоновано інтеграцію у програмне забезпечення за допомогою бібліотеки \
та віртуального блокового пристрою. Розглянуто підходи та структури даних які в визначених випадках роблять віддалену памʼять більш ефективною."),
        paragraph("Для забезпечення відмовостійкості, проаналізовано існуючі підходи до відновлення даних у разі відмови віддалених вузлів у існуючих реалізаціях віддаленої \
памʼяті. Зазначено їх переваги та недоліки. Підхід з використанням кодування стиранням як найбільш ефективний обрано для використання в цій роботі. Інші розглянуті методи \
надаються до використання опціонально, за конфігурацією від користувача."),
        paragraph("Крім цього, розглянуто проблему забезпечення високого рівня швидкодії під час доступу до даних у віддаленій памʼяті. Визначено які фактори впливають на \
затримку доступу і принципи, на яких ґрунтуються підходи для її зниження. Описано особливості реалізації клієнта віддаленої памʼяті а також протоколу передачі даних, які \
є важливими для забезпечення високого рівня швидкодії. Обґрунтовано необхідність використання переміщення проміжків памʼяті у фоновому потоці, вплив алгоритму заміщення \
проміжків на ефективність роботи віддаленої памʼяті. Розглянуто можливі алгоритми заміщення проміжків, описано механізм збору статистики доступу до проміжків \
памʼяті та її використання для неперевної адаптації параметрів моделі прогнозування у більш ефективних алгоритмах заміщення проміжків."),
        paragraph("Реалізація віддаленої памʼяті, яка розглядається в цій роботі, складається з методів вирішення кожної з підзадач, які було сформульовано та \
проаналізовано в цьому розділі."),
    ])
}
