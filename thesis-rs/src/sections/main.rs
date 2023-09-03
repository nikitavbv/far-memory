use {
    docx_rs::{Docx, Paragraph, LineSpacing, Run, Tab, AlignmentType, AbstractNumbering, Level, Start, NumberFormat, LevelText, LevelJc, SpecialIndentType, Numbering, NumberingId, IndentLevel},
    crate::{
        components::{SectionHeaderComponent, UnorderedListComponent},
        context::{Context, SectionContext},
    },
};

pub trait MainSection {
    fn add_main_section(self, context: &mut Context) -> Self;
}

impl MainSection for Docx {
    fn add_main_section(self, context: &mut Context) -> Self {
        let section_index = context.next_section_index();
        let mut section_context = SectionContext::new();

        let resource_type_numbering = context.next_numbering_id();

        self.add_section_header_placeholder_component(
            format!("{}  Аналіз проблеми", section_index).to_uppercase(), 
            "check how this section should be named properly"
        )
        .add_paragraph(
            Paragraph::new()
                .add_tab(Tab::new().pos(710))
                .line_spacing(LineSpacing::new().before(300).line(24 * 15))
                .style("Heading2")
                .add_run(Run::new().add_tab().add_text(format!("{}.{}   ", section_index, section_context.next_subsection_index())).add_text("Ресурси обладнання у розподілених системах та проблема їх ефективного використання"))
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
    }
}