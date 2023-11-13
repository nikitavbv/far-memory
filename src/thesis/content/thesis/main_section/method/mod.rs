use crate::thesis::engine::{Block, section_header, subsection_header, paragraph, SubsectionHeaderBlock};

mod components;
mod integration;
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

        subsection_header("Забезпечення швидкодії віддаленої памʼяті"),
        paragraph("Головним недоліком використання віддаленої памʼяті є негативний ефект на швидкодію програмного забезпечення, у яке вона інтегрована. Погіршення швидкодії виникає через більш \
повільний доступ до памʼяті який зумовлений в першу чергу використанням пристроїв зберігання (таких як памʼять інших вузлів, доступ до якої виконується за мережею, дисків, та ін.) затримка \
(latency) та пропускна здатність (bandwidth) яких є значно більшою за такі параметри для оперативної памʼяті. Крім цього, під час доступу до проміжку даних у віддаленій памʼяті виконуються \
додаткові перевірки та дії, які не є потрібними при роботі зі звичайною памʼяттю."),
        paragraph("Неможливо зробити доступ до віддаленої памʼяті таким саме швидким як доступ до оперативної памʼяті (у умовах, коли пристрій зберігання є повільним). Але має сенс мінімізувати \
затримку доступу до даних до того рівня, який є прийнятним для використання на практиці. Існує баланс між тим наскільки активно віддалена памʼять використовується програмним забезпеченням (\
скільки даних та якого типу в ній зберігається) та негативним ефектом на швидкодію програмного забезпечення. Доцільність використання віддаленої памʼяті та параметрів її роботи є відповідальністю \
розробника прикладного програмного забезпечення. Конфігурація віддаленої памʼяті обирається розробником базуючись на вимогах щодо швидкодії програмного забезпечення, його особливостях роботи \
з памʼяттю та характеристках апаратного забезпечення (наприклад, швидкість мережі)."),
        paragraph("Як було зазначено раніше, проміжки у віддаленій памʼяті ..."),
        // tell about optimizing network requests (why TCP (also, why nodelay is used and duplex) is used and not UDP, or http or some kind of existing RPC implementation). Tell about latency/bandwidth and userspace networking (it is not needed).
        // tell about reasoning behind partial swap in/swap out. tell why compression is not used. tell why copies should be avoided. tell a bit about size classes.
        // tell about background swap in and swap out threads and how synchronization should be performed.
        // explain what is the key in minimizing latency (keeping all the needed memory locally and moving it quickly) - like explained in the docs.
        // tell about policies to evict and pre-fetch spans (and how those use stats collected, heuristics, FSM, ML models, including RNN). explain why grouping objects in spans is effective. explain why it is important to reduce fragmentation and how it can be
        // achieved. tell about compaction.
        // explain what typical performance numbers are in various environments.
        // tell about page placement algorithms
        // explain how different software accesses memory. Tell how "ideal" policy works. Tell why "least recently used" can be a bad policy in some cases.
        // tell how performance is analyzed using tracing.

        Block::SubsectionHeader(SubsectionHeaderBlock::without_numbering("Висновки до розділу".to_owned())),
        // general conclusions

        // in third section explain that a demo app was implemented to measure how well everything works and the hardware of the test environment.
    ])
}
