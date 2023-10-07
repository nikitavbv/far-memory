use {
    tracing::warn,
    docx_rs::{Docx, Style, StyleType, RunFonts, PageMargin},
    crate::thesis::{
        engine::{Block, paragraph, unordered_list, count_pages, count_images, count_tables, PageCountingError},
        content::{Language, AbstractContent, Content},
        utils::mm_to_twentieth_of_a_point,
    },
    self::main_section::main_section,
};

mod main_section;

pub fn thesis_content(content: &Content) -> Block {
    let main = main_section();

    let abstract_placeholder_content = AbstractContent {
        total_pages: 42,
        total_images: count_images(&main),
        total_tables: count_tables(&main),
    };
    let content_with_placeholders = thesis_content_inner(abstract_placeholder_content.clone());

    let true_total_pages = match count_pages(thesis_docx_template(), content, &content_with_placeholders) {
        Ok(v) => v - 1, // front page does not count
        Err(err) => match err {
            PageCountingError::NoPdfConverterInstalled => {
                warn!("Cannot count pages, because pdf converter tool is not installed. Using \"0\" as the number of pages.");
                0
            }
        }
    }; 

    thesis_content_inner(AbstractContent {
        total_pages: true_total_pages,
        ..abstract_placeholder_content
    })
}

fn thesis_content_inner(abstract_content: AbstractContent) -> Block {
    /*
    requirements: https://ela.kpi.ua/bitstream/123456789/49978/1/Mahisterska_dysertatsiia.pdf
    examples: https://ela.kpi.ua/handle/123456789/21930
    */
    Block::Multiple(vec![
        // front page
        Block::FrontPage,

        // task
        Block::TaskSection,

        // abstract
        Block::AbstractSection(Language::Ukrainian, abstract_content.clone()),
        Block::AbstractSection(Language::English, abstract_content),

        // table of contents
        Block::Placeholder(Box::new(Block::SectionHeader("Зміст".to_uppercase())), "remove numbering".to_owned()),
        Block::TableOfContents,

        // abbreviations
        Block::Placeholder(Box::new(Block::SectionHeader("Перелік умовних позначень".to_uppercase())), "remove numbering".to_owned()),
        Block::Placeholder(
            Box::new(Block::Multiple(vec![
                paragraph("JDBC – прикладний програмний інтерфейс Java, який визначає методи, з допомогою яких програмне забезпечення на Java здійснює доступ до бази даних;"),
                paragraph("Cache – проміжний буфер з швидким доступом, що містить інформацію, яка може бути запрошена з найбільшою ймовірністю.")
            ])),
            "replace with real abbreviations".to_owned(),
        ),

        // intro
        Block::Placeholder(Box::new(Block::SectionHeader("Вступ".to_uppercase())), "remove numbering".to_owned()),
        paragraph("У сучасному світі дуже поширеним є хмарне програмне забезпечення, яке з кожним днем замінює собою або інтегрується у вигляді нового функціоналу у існуюче програмне забезпечення в усіх галузях використання. Центральним компонентом такого програмного забезпечення є його серверна частина, що обслуговує запити багатьох користувачів. Цей компонент обробляє велику кількість запитів від різних користувачів зазвичай виконуючи найбільш ресурсоємну частину роботи у порівнянні з частиною розміщенною на пристрої кінцевого користувача. Оскільки ці ресурси зазвичай обмежені можливостями обладнання, що використовується (чи бюджетом на оренду такого обладнання), то будь-яка оптимізація використання ресурсів призводить до можливості обробляти більшу кількість запитів та тому ж самому обладнанні (що в результаті знижує витрати)."),
        paragraph("Оператори великих центрів обробки даних вже великий час застосовують різні методи для підвищення ефективності використання ресурсів серверного обладнання. Так, наприклад, для ефективного використання ресурсів процесору використовується підхід “надмірної підписки” (oversubscription) обчислювального часу. Схожий метод використовується і при організації інфраструктури сховищ даних в додачу до компресії та дедублікації даних."),
        paragraph("Якщо перейти до ефективності використання оперативної памʼяті, то оператори найбільших у світі центрів обробки даних зазначають, що середнє використання памʼяті знаходиться на рівні близько 60%. Для того, щоб покращити цей показник розробляються різні методи. Одним з цих методів є використання віддаленої памʼяті (Far Memory)."),
        paragraph("Cервери у центрі обробки данних (і програмне забезпечення, що на них розгорнуте) можна поділити на два типи:"),
        unordered_list(vec![
            "сервери, на яких більша частина памʼяті є вільною".to_owned(),
            "сервери, які могли б цю памʼять використовувати, якщо мали би до неї доступ".to_owned(),
        ]),
        paragraph("Суть методу віддаленої памʼяті полягає в тому, що сервери з вільною памʼяттю можуть надавати доступ до неї по мережі тому програмному забезпеченню, яке могло б її використовувати для зберігання тієї частини даних, що підходить для зберігання за умов та обмежень, що накладає віддалена памʼять."),
        paragraph("У даній роботі розглянуто методи надання програмно-визначеної віддаленої памʼяті у розподілених системах, а також способи зниження затримки доступу до даних у віддаленій памяʼті та забезпечення відмовостійкості."),

        // main
        main_section(),

        Block::Placeholder(
            Box::new(Block::Multiple(vec![
                Block::Placeholder(Box::new(Block::SectionHeader("Висновок".to_owned())), "remove numbering".to_owned()),
                paragraph("Як підсумок проведеного аналізу проблеми, існуючих досліджень та реалізації та розробки архітектури архітектури програмного рішення програмно-визначеної віддаленої памʼяті, що розглядається у межах цієї роботи, було зроблено декілька висновків."),
                paragraph("По-перше, реалізація кластеру віддаленої памʼяті повинна містити наступні компоненти: сервіс керування кластером, сервіс зберігання даних та клієнтська інтеграція. Ці компоненти пересилають блоки памʼяті мережею для переміщення холодних сторінок памʼяті у віддалену памʼять та у зворотному порядку."),
                paragraph("По-друге, було встановлено, що найбільш оптимальним методом інтеграції в клієнтське програмне забезпечення є створення бібліотеки яка надає розробникам функції та структури даних для використання в своєму програмного забезпеченні. Також, оскільки велика частка програмного забезпечення не може змінюватись або не підходить до інтеграції з клієнтською бібліотекою за будь-яких причин, було досліджено та реалізовано у архітектурі альтернативний шлях реалізації: за допомогою віртуального блокового пристрою створеного за допомогою відповідного функціоналу операційної системи Linux."),
                paragraph("По-третє, були визначені та додані в архітектуру засоби забезпечення відмовостійкості системи та низької затримки операцій читання та запису у віддалену памʼять."),
                paragraph("В подальшому, розроблені вимоги та архітектура будуть використані для реалізації програмного рішення, його тестування та впровадження."),
            ])),
            "improve conclusions".to_owned(),
        ),

        // references
        Block::Placeholder(Box::new(Block::SectionHeader("Перелік посилань".to_uppercase())), "remove numbering".to_owned()),
        Block::ReferencesList(vec![
            "Carbink: Fault-tolerant Far Memory [Електорнний ресурс] // Yang Zhou Hassan Wassel Sihang Liu Jiaqi Gao James Mickens Minlan Yu Chris Kennelly Paul Jack Turner David E Culler Hank Levy Amin Vahdat - Proceedings of the 16th USENIX Symposium on Operating Systems Design and Implementation, Usenix - 2022. Режим доступу до ресурсу: https://research.google/pubs/pub51559/".to_owned(),
            "Software-Defined Far Memory in Warehouse-Scale Computers [Електронний ресурс] // Andres Lagar-Cavilla, Junwhan Ahn, Suleiman Souhlal, Neha Agarwal, Radoslaw Burny, Shakeel Butt, Jichuan Chang, Ashwin Chaugule, Nan Deng, Junaid Shahid, Greg Thelen, Kamil Adam Yurtsever, Yu Zhao, and Parthasarathy Ranganathan - International Conference on Architectural Support for Programming Languages and Operating Systems - 2019. Режим доступу до ресурсу: https://research.google/pubs/pub48551/".to_owned(),
            "AIFM: High-Performance, Application-Integrated Far Memory [Електронний ресурс] // Zhenyuan Ruan, MIT CSAIL; Malte Schwarzkopf, Brown University; Marcos K. Aguilera, VMware Research; Adam Belay, MIT CSAIL - 14th USENIX Symposium on Operating Systems Design and Implementation (OSDI 20) - 2020. Режим доступу до ресурсу: https://www.usenix.org/conference/osdi20/presentation/ruan".to_owned(),
            "Block Device Driver [Електорнний ресурс] // Linux Kernel Teaching. Режим доступу до ресурсу: https://linux-kernel-labs.github.io/refs/heads/master/index.html".to_owned(),
            "Understanding InfiniBand and RDMA [Електронний ресурс] // Red Hat Customer Portal. Режим доступу до ресурсу: https://access.redhat.com/documentation/en-us/red_hat_enterprise_linux/8/html/configuring_infiniband_and_rdma_networks/understanding-infiniband-and-rdma_configuring-infiniband-and-rdma-networks".to_owned(),
        ])
    ])
}

pub fn thesis_docx_template() -> Docx {
    // formatting: https://drive.google.com/file/d/1XzGVVvXRREoc6HGYMpjZFywzsWzRa01l/view
    Docx::new()
        .page_margin(
            PageMargin::new()
                .left(mm_to_twentieth_of_a_point(30.0))
                .top(mm_to_twentieth_of_a_point(20.0))
                .bottom(mm_to_twentieth_of_a_point(20.0))
                .right(mm_to_twentieth_of_a_point(10.0))
        )
        .default_fonts(RunFonts::new().cs("Times New Roman"))
        .default_size(28) // 14
        .default_tab_stop(0)
        .add_style(Style::new("Heading1", StyleType::Paragraph).name("Heading 1").bold())
        .add_style(Style::new("Heading2", StyleType::Paragraph).name("Heading 2").bold())
}