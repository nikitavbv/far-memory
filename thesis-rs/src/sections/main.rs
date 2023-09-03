use {
    docx_rs::{Docx, Paragraph, LineSpacing, Run, Tab, AlignmentType},
    crate::{
        components::{SectionHeaderComponent, UnorderedListComponent, ImageComponent},
        context::Context,
    },
};

pub trait MainSection {
    fn add_main_section(self, context: &mut Context) -> Self;
}

impl MainSection for Docx {
    fn add_main_section(self, context: &mut Context) -> Self {
        let section_index = context.next_section_index();

        self.add_section_header_placeholder_component(
            format!("{}  Аналіз проблеми", section_index).to_uppercase(), 
            "check how this section should be named properly"
        )
        .add_paragraph(
            Paragraph::new()
                .add_tab(Tab::new().pos(710))
                .line_spacing(LineSpacing::new().before(300).line(24 * 15))
                .style("Heading2")
                .add_run(Run::new().add_tab().add_text(format!("{}.{}   ", section_index, context.next_subsection_index(section_index))).add_text("Ресурси обладнання у розподілених системах та проблема їх ефективного використання"))
        )
        .add_paragraph(
            Paragraph::new()
                .add_tab(Tab::new().pos(710))
                .line_spacing(LineSpacing::new().line(24 * 15))
                .align(AlignmentType::Both)
                .add_run(Run::new().add_tab().add_text("Будь-який сучасний центр обробки даних складається з великої кількості серверного та мережевого обладнання. На цьому обладнанні виконується програмне забезпечення, що обробляє запити від користувачів та може бути частинами розподілених систем."))
        )
        .add_paragraph(
            Paragraph::new()
                .add_tab(Tab::new().pos(710))
                .line_spacing(LineSpacing::new().line(24 * 15))
                .align(AlignmentType::Both)
                .add_run(Run::new().add_tab().add_text("Під час своєї роботи на цьому обладнанні, програмне забезпечення може використовувати наступні його ресурси:"))
        )
        .add_unordered_list_component(context, vec![
            "процесорний час".to_owned(),
            "оперативна памʼять".to_owned(),
            "постійна памʼять на різних типах сховища, таких як жорсткі диски, твердотільні накопичувачі на ін.".to_owned(),
            "спеціалізовані пристрої, такі як графічні прискорювачі".to_owned(),
        ])
        .add_paragraph(
            Paragraph::new()
                .add_tab(Tab::new().pos(710))
                .line_spacing(LineSpacing::new().line(24 * 15))
                .align(AlignmentType::Both)
                .add_run(Run::new().add_tab().add_text("Для кожного з цих ресурсів існує проблема їх ефективного використання та різні рішення для досягнення такої мети."))
        )
        .add_paragraph(
            Paragraph::new()
                .add_tab(Tab::new().pos(710))
                .line_spacing(LineSpacing::new().line(24 * 15))
                .align(AlignmentType::Both)
                .add_run(Run::new().add_tab().add_text("Один з методів який дозволяє підвищити ефективність використання ресурсів процесору є “надмінна підписка” (oversubscription) його обчислювального часу. Це означає що на одному процесорі запускається декілька різних програм або віртуальних машин, кожна з яких використовує його частину часу, а разом всі - використовують процесор майже весь час, при цьому розрахунок йде на те, що піки завантаженості цих програм не співпадають."))
        )
        .add_paragraph(
            Paragraph::new()
                .add_tab(Tab::new().pos(710))
                .line_spacing(LineSpacing::new().line(24 * 15))
                .align(AlignmentType::Both)
                .add_run(Run::new().add_tab().add_text("Через особливості того, як програмне забезпечення працює з оперативною памʼяттю, вона є найбільш складним ресурсом, ефективність використання якого можна було б підвищити. Одним з підходів, що останнім часом багато досліджується та розглядається операторами великих центрів обробки даних для інтеграції є віддалена памʼять (Far Memory)."))
        )
        .add_paragraph(
            Paragraph::new()
                .add_tab(Tab::new().pos(710))
                .line_spacing(LineSpacing::new().line(24 * 15))
                .align(AlignmentType::Both)
                .add_run(Run::new().add_tab().add_text("Суть цього методу полягає в тому, що сервери у центрі обробки данних (і програмне забезпечення, що на них розгорнуте) можна поділити на два типи:"))
        )
        .add_unordered_list_component(context, vec![
            "сервери, на яких більша частина памʼяті є вільною".to_owned(),
            "сервери, які могли б цю памʼять використовувати, якщо мали би до неї доступ".to_owned(),
        ])
        .add_paragraph(
            Paragraph::new()
                .add_tab(Tab::new().pos(710))
                .line_spacing(LineSpacing::new().line(24 * 15))
                .align(AlignmentType::Both)
                .add_run(Run::new().add_tab().add_text("Програмне забезпечення першого типу зазвичай має “вузьке місце” у ресурсах процесору (наприклад, виконує задачі кодування даних, або простого обміну даними), програмне забезпечення другого - у ресурсах памʼяті (зазвичай це аналіз великих масивів даних або просто у програмного забезпечення є деякий великий набір даних, який йому потрібен для роботи). Використання памʼяті диску для розширення основної памʼяті не є оптимальним - через великий час доступу (а в хмарній інфраструктурі в додаток до цього зазвичай диски не є локальними, а розміщені віддалено на локальній інфраструктурі). У порівнянні з часом доступу до диску час доступу до даних у памʼяті іншого серверу є значно меншим (хоча все ще більшим за той випадок, коли дані доступні локально)."))
        )
        .add_image_component(
            context, 
            section_index, 
            "images/image1.png", 
            "Схематичне зображення принципу роботи Far Memory"
        )
        .add_paragraph(
            Paragraph::new()
                .add_tab(Tab::new().pos(710))
                .line_spacing(LineSpacing::new().line(24 * 15))
                .align(AlignmentType::Both)
                .add_run(Run::new().add_tab().add_text("Це все робить використання такої віддаленої памʼяті привабливим для випадків, коли можна знайти сторінки памʼяті, доступ до яких відбувається порівняно не часто, перемістити їх у віддалену памʼять та звільнити місце для даних, доступ до яких відбувається частіше."))
        )
        .add_paragraph(
            Paragraph::new()
                .add_tab(Tab::new().pos(710))
                .line_spacing(LineSpacing::new().before(300).line(24 * 15))
                .style("Heading2")
                .add_run(Run::new().add_tab().add_text(format!("{}.{}   ", section_index, context.next_subsection_index(section_index))).add_text("Аналіз існуючих реалізацій віддаленої памʼяті"))
        )
        .add_paragraph(
            Paragraph::new()
                .add_tab(Tab::new().pos(710))
                .line_spacing(LineSpacing::new().line(24 * 15))
                .align(AlignmentType::Both)
                .add_run(Run::new().add_tab().add_text("Аналіз існуючих реалізацій віддаленої памʼяті має на меті проаналізувати існуючі реалізації, їх архітектуру, причини певних рішень. Ціллю є дізнатися які з вже досліджених підходів є ефективними та знайти відповіді на наступні дослідницькі питання:"))
        )
    }
}