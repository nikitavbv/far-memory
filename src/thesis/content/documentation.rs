use crate::thesis::engine::{Block, subsection_header, section_header, paragraph, ImageBlock, TextSpan};

pub fn documentation() -> Block {
    /*
    ideas for methods of integration (there does not seem to be other methods other than these two that I have read about):
    - smart pointers - preferred way.
      - it makes sense to follow the same approach as carbink with size classes for objects.
    - swap device.
      - for implementation, split it into spans in sequence.
    backends:
    - remote RAM (erasure coding should be a part of the backend implementation, because some backends may not need it).
    - SSD.

    ideas for demo app:
    - LLM inference. Object allocation is static and read-only (keep weights in far memory).
      - mlockall can be used to prevent swapping.

    ideas for improving latency:
    - track stats for spans, not for objects, because it is less overhead.
    - for now, keep span id assignment static.
    - record stats for spans (access time) - that will allow to perform offline simulations.
      - stat can be access time within a window.
      - swap in/out events.
        - it is probably possible to track each interaction with smart pointer/swap device.
    */

    /*struct FarMemory<T> {
      inner: T,
    }

    impl<T> FarMemory<T> {
      pub fn new(inner: T) -> Self {
        Self {
          inner,
        }
      }
    }

    use std::ops::Deref;
    impl<T> Deref for FarMemory<T> {
      type Target = T;

      fn deref(&self) -> &Self::Target {
          unimplemented!()
      }
    }

    struct ApplicationData;

    impl ApplicationData {
      pub fn new() -> Self {
        Self
      }

      fn do_something_with_data(&self) {
      }
    }

    fn using_far_memory_in_application() {
      // this is how application data is normally accessed in application.
      let local_data = ApplicationData::new();
      local_data.do_something_with_data();

      // application data placed in far memory, under FarMemory smart pointer. From now on, this data
      // is managed by far memory and swapped between local and remote memory transparently.
      let far_memory_data = FarMemory::new(local_data);

      // when data is accessed, it is automatically moved into local memory.
      far_memory_data.do_something_with_data();

      // when data is not accessed, it may be moved back to remote memory.
    }*/

    Block::Multiple(vec![
        section_header("far memory"),
        Block::Note(r#"Please note that most parts of documentation for this project are in Ukrainian because I am working on this in scope of my thesis at Kyiv Polytechnic Institute and I
need to be able to refer to this documentation when talking to thesis supervisors and other people from the university. I will probably add English translation later."#.to_owned()),
        
        section_header("Віддалена памʼять"),
        paragraph("Віддалена памʼять - тип памʼяті що знаходиться на віддалених вузлах у розподіленій системі."),
        
        section_header("Формалізація задачі"),
        paragraph("Для прикладного програмного забезпечення, в яке інтегрована віддалена памʼять, максимізувати частку даних що зберігається у віддаленій памʼяті при умові дотримання вимог швидкодії."),

        section_header("Підзадачі"),
        paragraph("Нижче наведені підзадачі які потрібно виіршити для реалізації віддаленої памʼяті у порядку їх важливості."),

        subsection_header("Зниження затримки доступу (latency)"),
        paragraph("Затримка доступу до памʼяті має прямий вплив на швидкодію програмного забезпечення, тому її потрібно мінімізувати. Час читання даних з оперативної памʼяті нижчий за час читання даних по мережі, тому зниження затримки базується на тому, що потрібні дані вчасно переміщуються з памʼяті віддалених вузлів до оперативної памʼяті."),
        Block::Image(ImageBlock::new("latency.jpg".to_owned(), "затримка доступу до даних у віддаленій памʼяті".to_owned())),
        paragraph("Способи зниження затримки, які можна розглянути для використання:"),
        paragraph(vec![
          TextSpan::Bold("- групування обʼєктів".to_owned()),
          " таким чином, щоб обʼєкти доступ до яких відбувається частіше, знаходились в \"гарячих сторінках (spans)\". Обʼєкти, доступ до яких відбувається рідше, попадають у \"холодні сторінки\". Таким чином, у локальній памʼяті знаходиться більше гарячих обʼєктів і кількість запитів до інших вузлів знижається.".into(),
          r#" Такий підхід використовується у Carbink, де окремий потік переміщує обʼєкти між локальними сторінками для більш ефективного групування."#.into(),
        ]),
        paragraph(vec![
          TextSpan::Bold("- запит сторінок наперед".to_owned()),
          r#". Наприклад у AIFM структури даних оптимізвані завчасно завантажувати наступні сторінки. Наприклад, у масиві або списку під час ітерації завантажується наступни сторінки."#.into(),
        ]),
        paragraph(vec![
          TextSpan::Bold("- зниження фрагментації".to_owned()),
          ". При більш щільному розміщенні обʼєктів у сторінках, кількість сторінок що потрібно держати у памʼяті знижується, що також позитивно впливає на затримку. У Carbink це вирішується за допомогою використання size classes для обʼєктів, як у TCMalloc. Крім цього, розповсюдженим підходом є compaction, тобто пересення обʼєктів з менш завантажених на більш завантажені сторінки.".into(),
        ]),
        paragraph(vec![
          r#"Існуючі реалізації спираються на прості еврістики: рахування кількості доступів до обʼєктів для їх групування, запит наступної сторінки для структури даних. Розвитком цього може бути використання більш складних моделей для керування групуванням обʼєктів,
переміщення сторінок у віддалену памʼять та з неї, вирішення проблеми фрагментації. Методи які слід розглянути: еврістичні підходи, ML моделі (у тому числі RNN) та ін."#.into(),
        ]),
        paragraph(vec![
          "Також привабливим є збір статистики під час роботи програмного забезпечення та оптимізація моделей у реальному часі на її основі. Зібрана статистика може використовуватись як для побудови моделей, оптимізації їх 
          гіперпараметрів під час роботи а також для оцінки якості роботи віддаленої памʼяті. Такий підхід використовується наприклад у \"Software-defined far memory in warehouse-scale computers\", де зібрана статистика 
          використовується для оптимізації параметрів zswap (віддалена памʼять у цьому випадку - памʼять на диску, а не на віддалених вузлах).".into(),
        ]),

        subsection_header("Забезпечення відмовостійкості"),
        paragraph(
          r#"Оскільки сторінки памʼяті зберігаються на віддалених вузлах, то віддалені вузли становляться частиною домену збою (failure domain) для програмного забезпечення, у яке інтегрована віддалена памʼять. Для того, щоб 
обмежити негативний вплив на надійність програмного забезпечення, можна використовувати наступні методи для сторінок памʼяті у віддаленій памʼяті:"#
        ),
        paragraph(vec![
          TextSpan::Bold("- копія памʼяті на диску".to_owned()),
          ". При відмові віддаленого вузла, відновлення даних відбувається з диску. Недоліком цього підходу є повільне відновлення, та необхідність доступу до диску.".into(),
        ]),
        paragraph(vec![
          TextSpan::Bold("- реплікація".to_owned()),
          ". Сторінки памʼяті копіюються на декілька вузлів. При відмові одного з них, дані відновлюються з будь-якого з інших. Недоліком є надмірне використання памʼяті (більше на фактор реплікації).".into(),
        ]),
        paragraph(vec![
          TextSpan::Bold("- кодування стиранням".to_owned()),
          " (erasure coding). Наприклад, використовується код Ріда-Соломона для кодування сторінки у 5 частин (3 data shards, 2 parity shards). Ці частини даних розміщуються на різних вузлах, при виході з ладу будь-якого з них, дані можна відновити з інших вузлів. На відміну від реплікації, використовує менше памʼяті для забезпечення надлишковості для відновлення. Кількість частин даних може бути обрана користувачем в залежності від вимог до відмовостікйості.".into(),
        ]),
        Block::Image(ImageBlock::new("shards.jpg".to_owned(), "розміщення частин данних на різних вузлах при використанні erasure coding".to_owned())),
        paragraph("Розміщення частин даних на різних вузлах також дозволяє знизити час доступу до данних, оскільки достатньо отримати дані лише з частини вузлів для відновлення сторінки памʼяті у локальній памʼяті."),

        subsection_header("Інтеграція у існуюче та нове програмне забезпечення"),
        paragraph(
          "Для інтеграції у нове програмне забезпечення (те, де є можливість змінювати реалізацію) доцільним є використання розумних показчиків (з існуючих реалізацій так робить Carbink та AIFM). В межах цієї роботи створюється бібліотека на мові програмування Rust, 
          яка надає можливість розробнику обирати які дані будуть зберігатися у віддаленій памʼяті. Бібліотека керує переміщенням даних у та з віддаленої памʼяті автоматично. Створення реалізацій структур даних призначених для
          роботи з віддаленою памʼяттю (як у AIFM) не розглядається, оскільки їх використання можна уникнути, якщо автоматичне завантаження сторінок паʼяті працює достатньо ефективно."
        ),
        Block::Image(ImageBlock::new("integration.png".to_owned(), "використання розумного показчика для розміщення даних у віддаленій памʼяті".to_owned())),
        paragraph(
          "Для інтеграції у існуюче програмне забезпечення, або те, яке написане на інших мовах програмування можна використовувати механізм підкачки (swapping) памʼяті у операційній системі. На відміну від звичайного swap, який 
          розміщується на диску, в цьому випадку swap розміщується на віртуальному блоковому пристрої, блоки якого відповідають сторінкам у віддаленій памʼяті. Реалізація блокового пристрою використовує ту ж реалізацію переміщення
          сторінок між локальною та віддаленою памʼяттю, що і для інтеграції на основі розмуних показчиків."
        )
    ])
}
