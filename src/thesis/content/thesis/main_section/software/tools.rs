use crate::thesis::engine::{Block, subsection_header, paragraph, Reference, reference, TextSpan};

pub fn tools() -> Block {
    Block::Multiple(vec![
        subsection_header("Засоби розробки"),
        paragraph("Для реалізації усіх компонентів віддаленої памʼяті було обрано мову програмування Rust. Rust є популярною мовою у сфері \
системного програмування та є ефективною для цієї задачі так як ця мова є компільованою, не має збирача сміття, використовує абстракції з \
нульовою ціною що дозволяє отримати рівень швидкодії що є подібним до рівня який забезпечують такі мови програмування як C чи C++. При цьому\
, Rust гарантує безпечну роботу з памʼяттю завдяки системі статичної перевірки посилань (borrow checker), що спрощує написання безпечного \
програмного забезпечення та паралельних обчислень. Крім цього, Rust є сучасною мовою програмування що підтримує функціональну парадигму \
програмування, має строгу типізацію, що спрощує розробку програмного забезпечення. Розвинена екосистема цієї мови дає можливість \
використовувати якісні бібліотеки для типових задач, де це необхідно."),
        paragraph(vec![
            "Клієнт віддаленої памʼяті використовує декілька потоків виконання, то виникає проблема у їх синхронізації з використанням \
спеціалізованих структур даних та інструментів синхронізації виконання. Оскільки стандартної бібліотеки мови програмування Rust в деяких \
випадках може бути недостатньо, то використовується бібліотека ".into(),
            reference_to_rust_crate("crossbeam"),
            " що містить додаткові інструменти для паралельного програмування.".into(),
        ]),
        paragraph(vec![
            "В деяких компонентах виникає потреба у використанні асинхронних функцій (наприклад, у сервері зберігання та сервері \
керування). Це дозволяє спростити реалізацію коду, що паралельно працює з декількома мережевими зʼєднаннями (чи іншими IO операціями), \
та зробити її більш ефективною у порівнянні з кодом що використовує окремі потоки рівня операційної системи. В мові програмування Rust \
для виконання асинхронних функцій потрібно використати середовище виконання. В цій роботі використовується ".into(),
            reference_to_rust_crate("tokio"),
            " який є найбільш популярною \
бібліотекою що надає середовище виконання та реалізації асинхронних функцій.".into(),
        ]),
        paragraph(vec![
            "Для того, щоб мати можливість детально аналізувати роботу віддаленої памʼяті та шукати вузькі місця які вимагають \
оптимізацій необхідним є фреймворк інструментування програмування забезпечення з підтримкою трасування, оскільки звичайного логування \
недостатньо. У цій роботі для цього використовується бібліотека ".into(),
            reference_to_rust_crate("tracing"),
            ". У поєднанні з ".into(),
            reference_to_rust_crate("tracing-chrome"),
            " вона дозволяє зберігати інформацію про події під час роботи клієнту віддаленої памʼяті та аналізувати їх у вигляді діаграми з часовою шкалою за допомогою ".into(),
            reference("chrome developer tools", Reference::for_website("The Trace Event Profiling Tool (about:tracing)", "https://www.chromium.org/developers/how-tos/trace-event-profiling-tool/")),
            ".".into()
        ]),
        paragraph(vec![
            "Бібліотека ".into(),
            reference_to_rust_crate("vblk"),
            " використана для реалізації віртуального блокового пристрою. Для цього ця бібліотека взаємодіє з модулем ".into(),
            reference("NBD", Reference::for_website("Network Block Device // The Linux Kernel documentation".to_owned(), "https://docs.kernel.org/admin-guide/blockdev/nbd.html".to_owned())),
            " у операційній системі Linux.".into(),
        ]),
        paragraph(vec![
            "Для серіалізації даних використовується бібліотека ".into(),
            reference_to_rust_crate("serde"),
            " разом з bincode, що реалізує компактне кодування даних у набір \
байт. За замовчуванням serde працює неефективно з векторами байт (Vec<u8>): серіалізує кожен елемент окремо замість того, щоб скопіювати \
усю ділянку памʼяті за одну операцію. Для усунення цього недоліку використовується бібліотека ".into(),
            reference_to_rust_crate("serde-bytes"),
            ".".into(),
        ]),
        paragraph(vec![
            "Для реалізацій бекендів клієнта віддаленої памʼяті використані наступні бібліотеки: ".into(),
            reference_to_rust_crate("reed-solomon-erasure"),
            " (для кодування та відновлення даних кодами Ріда-Соломона), ".into(),
            reference_to_rust_crate("aes-gcm"),
            " (для шифрування), ".into(),
            reference_to_rust_crate("lz4"),
            " (для стиснення даних).".into(),
        ]),
        paragraph(vec![
            "Деякі реалізації алгоритмів заміщення проміжків памʼяті, що розглядаються в цій роботі, використовують алгоритми \
машинного навчання, наприклад рекурентні нейронні мережі. Для цього використана бібліотека ".into(),
            reference("candle", Reference::for_website("candle - Minimalist ML framework for Rust // Github".to_owned(), "https://github.com/huggingface/candle".to_owned())),
            ". Перевагою використання цієї бібліотеки \
є те, що вона повністю реалізована на мові програмування Rust та не містить зовнішніх залежностей (в тому числі не використовує динамічні \
бібліотеки, такі як libtorch). Це дозволяє зробити процес зборки та розгортання програмного забезпечення більш простим, так як результатом \
компіляції є виконуваний файл з усіма залежностями залінкованими статично.".into(),
        ]),
        paragraph(vec![
            "Бібліотека ".into(),
            reference_to_rust_crate("thiserror"),
            " використовується для простого визначення типів помилок, що використовуються у клієнті та іншних \
компонентах віддаленої памʼяті.".into(),
        ]),
        paragraph("Для моніторингу використовується бібліотека prometheus, за допомогою якої важливі показники, такі як кількість та тип \
проміжків памʼяті, швидкість їх передачі відслідковуються у лічильних. Отримані значення передаються раз на 10 секунд у базу даних сумісну \
з Prometheus методом push."),
        paragraph("Для того, щоб зробити розгортання системи більш простим для кінцевого користувача, збираються Docker зображення (в одному \
зображенні є всі компоненти віддаленої памʼяті, так як вони всі доступні у вигляді одного виконуваного файла що запускається з різними \
параметрами). Крім цього, реалізовані файли-визначення для Kubernetes, що дає можливість у разі необхідності розгорнути усі компоненти \
системи у Kubernetes кластері."),
    ])
}

fn reference_to_rust_crate(crate_name: &str) -> TextSpan {
    reference(
        crate_name,
        Reference::for_website(
            format!("{} // Rust Package Registry", crate_name),
            format!("https://crates.io/crates/{}", crate_name)
        )
    )
}
