use {
    tracing::warn,
    docx_rs::{Docx, Style, StyleType, RunFonts, PageMargin, Header, Paragraph, Run, InstrText, AlignmentType},
    crate::thesis::{
        engine::{
            Block,
            paragraph,
            unordered_list,
            count_pages,
            count_images,
            count_tables,
            count_applications,
            PageCountingError,
            count_references,
            section_header,
            SectionHeaderBlock,
            ParagraphBlock,
            TextSpan,
            Alignment,
            extract_references_text,
            ReferenceFormat,
            Reference,
            reference,
        },
        content::{Language, AbstractContent, Content},
        utils::mm_to_twentieth_of_a_point,
    },
    self::{
        main_section::main_section,
        abstract_section::abstract_section,
        applications::{applications, code_listing_application},
    },
};

mod main_section;

mod abstract_section;
mod applications;

pub fn thesis_content(content: &Content) -> Block {
    let main = main_section(true);
    let applications = applications();
    thesis_content_inner(generate_abstract_content(content, &main, &applications), true, main, Some(applications))
}

pub fn thesis_content_for_plagiarism_check() -> Block {
    thesis_body(main_section(false))
}

pub fn thesis_application_code_listing() -> Block {
    code_listing_application()
}

fn generate_abstract_content(content: &Content, main: &Block, applications: &Block) -> AbstractContent {
    let abstract_placeholder_content = AbstractContent {
        total_pages: 42,
        total_images: count_images(&main),
        total_tables: count_tables(&main),
        total_applications: count_applications(&applications),
        total_references: 42,
    };
    let content_with_placeholders = thesis_content_inner(abstract_placeholder_content.clone(), true, main.clone(), Some(applications.clone()));

    let true_total_pages = match count_pages(thesis_docx_template(true), content, &content_with_placeholders) {
        Ok(v) => v,
        Err(err) => match err {
            PageCountingError::NoPdfConverterInstalled => {
                warn!("Cannot count pages, because pdf converter tool is not installed. Using \"0\" as the number of pages.");
                0
            }
        }
    };

    AbstractContent {
        total_pages: true_total_pages,
        total_references: count_references(&content_with_placeholders),
        ..abstract_placeholder_content
    }
}

fn thesis_content_inner(abstract_content: AbstractContent, front_page: bool, main: Block, applications: Option<Block>) -> Block {
    let body = thesis_body(main);
    let references = extract_references_text(&ReferenceFormat::Thesis, &body);

    /*
    requirements: https://ela.kpi.ua/bitstream/123456789/49978/1/Mahisterska_dysertatsiia.pdf
    examples: https://ela.kpi.ua/handle/123456789/21930
    */
    Block::Multiple(vec![
        // front page
        if front_page { Block::FrontPage } else { Block::Multiple(vec![]) },

        // task
        Block::TaskSection,

        // abstract
        Block::AbstractSection(Language::Ukrainian, abstract_content.clone()),
        abstract_section(&Language::Ukrainian),
        Block::AbstractSection(Language::English, abstract_content),
        abstract_section(&Language::English),

        // table of contents
        Block::SectionHeader(SectionHeaderBlock::without_numbering("Зміст".to_uppercase()).do_not_include_in_table_of_contents()),
        Block::TableOfContents,

        // abbreviations
        Block::SectionHeader(SectionHeaderBlock::without_numbering("Перелік умовних позначень".to_uppercase()).do_not_include_in_table_of_contents()),
        Block::Multiple(vec![
            paragraph("TCP – Transmission Control Protocol, мережевий протокол транспортного рівня;"),
            paragraph("RDMA – Remote Direct Memory Access, технологія яка дозволяє вузам в системі працювати з даними у памʼяті інших вузлів, не використовуючи ресурси їх процесору;"),
            paragraph("HPC – High Performance Computing, підхід у вирішенні задач, які потребують багато обчислень за допомогою суперкомпʼютерів або обчислювальних кластерів;"),
            paragraph("MPI – Message Passing Interface, стандарт обміну інформацією в програмному забезпеченні, яке працює на обчислювальних кластерах;"),
            paragraph("zswap – функціонал ядра операційної системи Linux, що дозволяє стискати памʼять у файлах підкачки;"),
            paragraph("RAM - random-access memory, оперативна памʼять, у якій зберігаються код та дані програм;"),
            paragraph("latency - затримка часу між запитом до даних та повернення системою даних у відповідь;"),
            paragraph("span - сторінка або блок памʼяті, що є одиницею, з якою працює віддалена памʼять;"),
            paragraph("LRU - least recently used, алгоритм заміщення найменш нещодавно використаних елементів у памʼяті."),
        ]),

        body,

        // references
        Block::SectionHeader(SectionHeaderBlock::without_numbering("Список використаних джерел".to_owned())),
        Block::ReferencesList(references),

        // applications
        Block::Paragraph(ParagraphBlock::new(vec![
            TextSpan::PageBreak,
            TextSpan::Break, // yeah, this is ugly
            TextSpan::Break,
            TextSpan::Break,
            TextSpan::Break,
            TextSpan::Break,
            TextSpan::Break,
            TextSpan::Break,
            TextSpan::Break,
            TextSpan::Break,
            TextSpan::Break,
            TextSpan::Break,
            TextSpan::Break,
            TextSpan::Break,
            TextSpan::Break,
            TextSpan::Break,
            TextSpan::Break,
            TextSpan::Bold(Box::new("Додатки".to_uppercase().into())),
        ].into()).with_tab(false).with_alignment(Alignment::Center)),
        applications.unwrap_or(Block::Multiple(vec![]))
    ])
}

fn thesis_body(main: Block) -> Block {
    Block::Multiple(vec![
        introduction(),
        main,
        conclusions(),
    ])
}

fn introduction() -> Block {
    Block::Multiple(vec![
        Block::SectionHeader(SectionHeaderBlock::without_numbering("Вступ".to_uppercase())),
        paragraph("У сучасному світі дуже поширеним є хмарне програмне забезпечення, яке з кожним днем замінює собою або інтегрується у вигляді нового функціоналу у існуюче програмне забезпечення в усіх галузях використання. Центральним компонентом такого програмного забезпечення є його серверна частина, що обслуговує запити багатьох користувачів. Цей компонент обробляє велику кількість запитів від різних користувачів зазвичай виконуючи найбільш ресурсоємну частину роботи у порівнянні з частиною розміщеною на пристрої кінцевого користувача. Оскільки ці ресурси зазвичай обмежені можливостями обладнання, що використовується (чи бюджетом на оренду такого обладнання), то будь-яка оптимізація використання ресурсів призводить до можливості обробляти більшу кількість запитів та тому ж самому обладнанні (що в результаті знижує витрати)."),
        paragraph("Оператори великих центрів обробки даних вже великий час застосовують різні методи для підвищення ефективності використання ресурсів серверного обладнання. Так, наприклад, для ефективного використання ресурсів процесору використовується підхід “надмірної підписки” (oversubscription) обчислювального часу. Схожий метод використовується і при організації інфраструктури сховищ даних в додачу до компресії та дедублікації даних."),
        paragraph(vec![
            "Якщо перейти до ефективності використання оперативної памʼяті, то оператори найбільших у світі центрів обробки даних зазначають, що середнє використання памʼяті знаходиться на рівні близько ".into(),
            reference("60%", Reference::for_publication(
                "Borg: the Next Generation".to_owned(),
                "Muhammad Tirmazi, Adam Barker, Nan Deng, Md Ehtesam Haque, Zhijing Gene Qin, Steven Hand, Mor Harchol-Balter, John Wilkes".to_owned(),
                2020,
                "Proceedings of ACM EuroSys".to_owned()
            )),
            ". Для того, щоб покращити цей показник розробляються різні методи. Одним з цих методів є використання віддаленої памʼяті (Far Memory).".into(),
        ]),
        paragraph("Cервери у центрі обробки даних (і програмне забезпечення, що на них розгорнуте) можна поділити на два типи:"),
        unordered_list(vec![
            "сервери, на яких більша частина памʼяті є вільною".to_owned(),
            "сервери, які могли б цю памʼять використовувати, якщо мали би до неї доступ".to_owned(),
        ]),
        paragraph("Суть методу віддаленої памʼяті полягає в тому, що сервери з вільною памʼяттю можуть надавати доступ до неї по мережі тому програмному забезпеченню, яке могло б її використовувати для зберігання тієї частини даних, що підходить для зберігання за умов та обмежень, що накладає віддалена памʼять."),
        paragraph("У даній роботі розглянуто методи надання програмно-визначеної віддаленої памʼяті у розподілених системах, а також способи зниження затримки доступу до даних у віддаленій памʼяті та забезпечення відмовостійкості."),
    ])
}

fn conclusions() -> Block {
    Block::Multiple(vec![
        Block::SectionHeader(SectionHeaderBlock::without_numbering("Висновки".to_owned())),
        paragraph("У роботі було розвʼязано проблему підвищення ефективності використання оперативної памʼяті у центрах обробки даних за рахунок \
розробки ефективного методу надання віддаленої памʼяті в розподілених інформаційних системах."),
        paragraph("При цьому отримано наступні наукові та практичні результати:"),
        Block::UnorderedList(vec![
            "розроблено архітектуру програмного засобу для надання віддаленої памʼяті".to_owned(),
            "визначено метод інтеграції у програмне забезпечення за допомогою клієнтської бібліотеки що надає розумні покажчики для зберігання \
обʼєктів у віддаленій памʼяті та структури даних адаптовані для роботи з нею".to_owned(),
            "розроблено альтернативний метод інтеграції за рахунок надання віртуального блокового пристрою".to_owned(),
            "задача забезпечення відмовостійкості віддаленої памʼяті була вирішена за рахунок використання кодів Ріда-Соломона для кодування \
та розміщення частин даних на різних вузлах".to_owned(),
            "знижено в середньому затримку доступу до блоків у віддаленій памʼяті за рахунок використання \
фонового потоку виконання для завчасного переміщення проміжків та використання алгоритму заміщення проміжків що спирається на статистику доступу до памʼяті яка неперервно збирається під час роботи програмного забезпечення".to_owned(),
            "було створено програмний продукт для надання віддаленої памʼяті що реалізує запропонований метод".to_owned(),
            "ефективність запропонованого методу надання віддаленої памʼяті було доведено статистично. Було встановлено що для програмного забезпечення \
що має схеми доступу до памʼяті що дозволяють ефективно використовувати прогнозні моделі, при 40% локальної памʼяті запронований метод дозволяє \
досягти на 53% більшу пропускну здатність у порівняні з більш простими алгоритмами заміщення проміжків".to_owned(),
            "проведено маркетинговий аналіз стартап-проекту, що включає в себе опис ідеї, її технологічний аудит та аналіз ринкових можливостей \
запуску стартап-проекту. Розроблено ринкову стратегію проекту, маркетингову програму. Проведений аналіз показує що подальша імплементація проекту є \
доцільною".to_owned(),
        ]),
        paragraph("На основі матеріалів магістерської дисертації було опубліковано тези доповіді на V науково-практичній конференції молодих вчених \
та студентів «Інженерія програмного забезпечення і передові інформаційні технології SoftTech-2023»."),
        paragraph("Наукова новизна запропонованого методу надання віддаленої памʼяті полягає в тому, що, на відміну від існуючих методів, задача \
заміщення проміжків вирішена статистично більш ефективно за рахунок реалізації адаптації параметрів моделі прогнозування доступу на основі використання \
статистики, що неперервно збирається в процесі роботи програмного забезпечення."),
        paragraph("Практичне значення отриманих результатів полягає в тому, що розроблене програмне забезпечення для надання віддаленої памʼяті є простим для розгортання, не вимагає значних змін у програмне забезпечення при інтеграції. \
Дане програмне забезпечення може бути використане для підвищення ефективності використання ресурсів центру обробки даних у програмному забезпеченні параметри роботи якого дозволяють використання такого класу памʼяті як віддалена памʼять."),
    ])
}

pub fn thesis_docx_template(skip_front_page_from_numbering: bool) -> Docx {
    // formatting: https://drive.google.com/file/d/1XzGVVvXRREoc6HGYMpjZFywzsWzRa01l/view
    let docx = Docx::new()
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
        .header(
            Header::new()
                .add_paragraph(Paragraph::new()
                    .align(AlignmentType::Right)
                    .add_run(
                        Run::new()
                            .add_field_char(docx_rs::FieldCharType::Begin, false)
                            .add_instr_text(InstrText::Unsupported("PAGE".to_owned()))
                            .add_field_char(docx_rs::FieldCharType::End, false)
                    ))
        );

    let docx = if skip_front_page_from_numbering {
        docx.first_header(Header::new())
    } else {
        docx
    };

    docx.add_style(Style::new("Heading1", StyleType::Paragraph).name("Heading 1").bold())
        .add_style(Style::new("Heading2", StyleType::Paragraph).name("Heading 2").bold())
}

pub fn practice_report_content(content: &Content) -> Block {
    let applications = applications();
    let main = main_section(true);

    let abstract_placeholder_content = AbstractContent {
        total_pages: 42,
        total_images: count_images(&main),
        total_tables: count_tables(&main),
        total_applications: 42,
        total_references: 42,
    };
    let content_with_placeholders = thesis_content_inner(abstract_placeholder_content.clone(), true, main.clone(), None);

    let true_total_pages = match count_pages(thesis_docx_template(true), content, &content_with_placeholders) {
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
        total_applications: count_applications(&content_with_placeholders),
        total_references: count_references(&content_with_placeholders),
        ..abstract_placeholder_content
    }, false, main, None)
}
