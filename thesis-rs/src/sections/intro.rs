use {
    docx_rs::{Docx, Paragraph, LineSpacing, AlignmentType, Run, Tab, AbstractNumbering, Start, NumberFormat, LevelText, LevelJc, Numbering, SpecialIndentType, Level, NumberingId, IndentLevel},
    crate::{
        context::Context,
        components::PlaceholderComponent,
    },
};

pub trait IntroSection {
    fn add_intro_section(self, context: &mut Context) -> Self;
}

impl IntroSection for Docx {
    fn add_intro_section(self, context: &mut Context) -> Self {
        let server_types_numbering = context.next_numbering_id();

        self.add_paragraph(
            Paragraph::new()
                .line_spacing(LineSpacing::new().after(300))
                .style("Heading1")
                .page_break_before(true)
                .align(AlignmentType::Center)
                .add_run(Run::new().add_text("Вступ".to_uppercase()))
        ).add_paragraph(
            Paragraph::new()
                .add_tab(Tab::new().pos(710))
                .line_spacing(LineSpacing::new().line(24 * 15))
                .align(AlignmentType::Both)
                .add_run(Run::new().add_tab().add_text("У сучасному світі дуже поширеним є хмарне програмне забезпечення, яке з кожним днем замінює собою або інтегрується у вигляді нового функціоналу у існуюче програмне забезпечення в усіх галузях використання. Центральним компонентом такого програмного забезпечення є його серверна частина, що обслуговує запити багатьох користувачів. Цей компонент обробляє велику кількість запитів від різних користувачів зазвичай виконуючи найбільш ресурсоємну частину роботи у порівнянні з частиною розміщенною на пристрої кінцевого користувача. Оскільки ці ресурси зазвичай обмежені можливостями обладнання, що використовується (чи бюджетом на оренду такого обладнання), то будь-яка оптимізація використання ресурсів призводить до можливості обробляти більшу кількість запитів та тому ж самому обладнанні (що в результаті знижує витрати)."))
        ).add_paragraph(
            Paragraph::new()
                .add_tab(Tab::new().pos(710))
                .line_spacing(LineSpacing::new().line(24 * 15))
                .align(AlignmentType::Both)
                .add_run(Run::new().add_tab().add_text("Оператори великих центрів обробки даних вже великий час застосовують різні методи для підвищення ефективності використання ресурсів серверного обладнання. Так, наприклад, для ефективного використання ресурсів процесору використовується підхід “надмірної підписки” (oversubscription) обчислювального часу. Схожий метод використовується і при організації інфраструктури сховищ даних в додачу до компресії та дедублікації даних."))
        ).add_paragraph(
            Paragraph::new()
                .add_tab(Tab::new().pos(710))
                .line_spacing(LineSpacing::new().line(24 * 15))
                .align(AlignmentType::Both)
                .add_run(Run::new().add_tab().add_text("Якщо перейти до ефективності використання оперативної памʼяті, то оператори найбільших у світі центрів обробки даних зазначають, що середнє використання памʼяті знаходиться на рівні близько 60%. Для того, щоб покращити цей показник розробляються різні методи. Одним з цих методів є використання віддаленої памʼяті (Far Memory)."))
        ).add_paragraph(
            Paragraph::new()
                .add_tab(Tab::new().pos(710))
                .line_spacing(LineSpacing::new().line(24 * 15))
                .align(AlignmentType::Both)
                .add_run(Run::new().add_tab().add_text("Cервери у центрі обробки данних (і програмне забезпечення, що на них розгорнуте) можна поділити на два типи:"))
        ).add_abstract_numbering(
            AbstractNumbering::new(server_types_numbering)
                .add_level(Level::new(
                    0,
                    Start::new(0),
                    NumberFormat::new("bullet"),
                    LevelText::new("– "),
                    LevelJc::new("left")
                ).indent(None, Some(SpecialIndentType::FirstLine(725)), None, None))
        )
        .add_numbering(Numbering::new(server_types_numbering, server_types_numbering))
        .add_paragraph(
            Paragraph::new()
                .add_tab(Tab::new().pos(710))
                .line_spacing(LineSpacing::new().line(24 * 15))
                .align(AlignmentType::Both)
                .numbering(NumberingId::new(server_types_numbering), IndentLevel::new(0))
                .add_run(Run::new().add_text("сервери, на яких більша частина памʼяті є вільною;"))
        ).add_paragraph(
            Paragraph::new()
                .add_tab(Tab::new().pos(710))
                .line_spacing(LineSpacing::new().line(24 * 15))
                .align(AlignmentType::Both)
                .numbering(NumberingId::new(server_types_numbering), IndentLevel::new(0))
                .add_run(Run::new().add_text("сервери, які могли б цю памʼять використовувати, якщо мали би до неї доступ."))
        ).add_paragraph(
            Paragraph::new()
                .add_tab(Tab::new().pos(710))
                .line_spacing(LineSpacing::new().line(24 * 15))
                .align(AlignmentType::Both)
                .add_run(Run::new().add_tab().add_text("Суть методу віддаленої памʼяті полягає в тому, що сервери з вільною памʼяттю можуть надавати доступ до неї по мережі тому програмному забезпеченню, яке могло б її використовувати для зберігання тієї частини даних, що підходить для зберігання за умов та обмежень, що накладає віддалена памʼять."))
        ).add_paragraph(
            Paragraph::new()
                .add_tab(Tab::new().pos(710))
                .line_spacing(LineSpacing::new().line(24 * 15))
                .align(AlignmentType::Both)
                .add_run(Run::new().add_tab())
                .add_placeholder_component("Завданням цього курсового проекту було поставлено аналіз проблеми, її існуючих рішень, формалізація методу та розробка архітектури програмного забезпечення для надання програмно-визначеної віддаленої памʼяті у розподілених системах.", "mentions coursework")
        )
    }
}