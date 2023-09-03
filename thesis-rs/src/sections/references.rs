use {
    docx_rs::{Docx, AbstractNumbering, Level, Start, NumberFormat, LevelText, LevelJc, Numbering, Paragraph, LineSpacing, NumberingId, IndentLevel, AlignmentType, Run, SpecialIndentType},
    crate::{
        components::{SectionHeaderComponent, ParagraphComponent},
        context::Context,
    },
};

pub trait ReferencesSection {
    fn add_references_section(self, context: &mut Context) -> Self;
}

impl ReferencesSection for Docx {
    fn add_references_section(self, context: &mut Context) -> Self {
        let numbering = context.next_numbering_id();

        self
            .add_section_header_component("Перелік посилань")
            .add_abstract_numbering(
                AbstractNumbering::new(numbering)
                    .add_level(Level::new(
                        0,
                        Start::new(1),
                        NumberFormat::new("decimal"),
                        LevelText::new("%1. "),
                        LevelJc::new("start")
                    ).indent(None, Some(SpecialIndentType::Hanging(300)), None, None)
                )
            )
            .add_numbering(Numbering::new(numbering, numbering))
            .add_paragraph(Paragraph::new()
                .line_spacing(LineSpacing::new().before(150))
                .numbering(NumberingId::new(numbering), IndentLevel::new(0))
                .align(AlignmentType::Both)
                .add_run(Run::new().add_text("Software-Defined Far Memory in Warehouse-Scale Computers [Електронний ресурс] // Andres Lagar-Cavilla, Junwhan Ahn, Suleiman Souhlal, Neha Agarwal, Radoslaw Burny, Shakeel Butt, Jichuan Chang, Ashwin Chaugule, Nan Deng, Junaid Shahid, Greg Thelen, Kamil Adam Yurtsever, Yu Zhao, and Parthasarathy Ranganathan - International Conference on Architectural Support for Programming Languages and Operating Systems - 2019. Режим доступу до ресурсу: https://research.google/pubs/pub48551/"))
            )
            .add_paragraph(Paragraph::new()
                .line_spacing(LineSpacing::new().before(150))
                .numbering(NumberingId::new(numbering), IndentLevel::new(0))
                .align(AlignmentType::Both)
                .add_run(Run::new().add_text("Carbink: Fault-tolerant Far Memory [Електорнний ресурс] // Yang Zhou Hassan Wassel Sihang Liu Jiaqi Gao James Mickens Minlan Yu Chris Kennelly Paul Jack Turner David E Culler Hank Levy Amin Vahdat - Proceedings of the 16th USENIX Symposium on Operating Systems Design and Implementation, Usenix - 2022. Режим доступу до ресурсу: https://research.google/pubs/pub51559/"))
            )
    }
}