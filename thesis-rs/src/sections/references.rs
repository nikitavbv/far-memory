use {
    docx_rs::{Docx, AbstractNumbering, Level, Start, NumberFormat, LevelText, LevelJc, Numbering, Paragraph, LineSpacing, NumberingId, IndentLevel, AlignmentType, Run, SpecialIndentType},
    crate::{
        components::SectionHeaderComponent,
        context::Context,
    },
};

pub trait ReferencesSection {
    fn add_references_section(self, context: &mut Context) -> Self;
}

impl ReferencesSection for Docx {
    fn add_references_section(self, context: &mut Context) -> Self {
        self
            .add_section_header_component("Перелік посилань")
            .add_references_list_component(context, &[
                "Carbink: Fault-tolerant Far Memory [Електорнний ресурс] // Yang Zhou Hassan Wassel Sihang Liu Jiaqi Gao James Mickens Minlan Yu Chris Kennelly Paul Jack Turner David E Culler Hank Levy Amin Vahdat - Proceedings of the 16th USENIX Symposium on Operating Systems Design and Implementation, Usenix - 2022. Режим доступу до ресурсу: https://research.google/pubs/pub51559/",
                "Software-Defined Far Memory in Warehouse-Scale Computers [Електронний ресурс] // Andres Lagar-Cavilla, Junwhan Ahn, Suleiman Souhlal, Neha Agarwal, Radoslaw Burny, Shakeel Butt, Jichuan Chang, Ashwin Chaugule, Nan Deng, Junaid Shahid, Greg Thelen, Kamil Adam Yurtsever, Yu Zhao, and Parthasarathy Ranganathan - International Conference on Architectural Support for Programming Languages and Operating Systems - 2019. Режим доступу до ресурсу: https://research.google/pubs/pub48551/",
                "AIFM: High-Performance, Application-Integrated Far Memory [Електронний ресурс] // Zhenyuan Ruan, MIT CSAIL; Malte Schwarzkopf, Brown University; Marcos K. Aguilera, VMware Research; Adam Belay, MIT CSAIL - 14th USENIX Symposium on Operating Systems Design and Implementation (OSDI 20) - 2020. Режим доступу до ресурсу: https://www.usenix.org/conference/osdi20/presentation/ruan",
                "Block Device Driver [Електорнний ресурс] // Linux Kernel Teaching. Режим доступу до ресурсу: https://linux-kernel-labs.github.io/refs/heads/master/index.html",
                "Understanding InfiniBand and RDMA [Електронний ресурс] // Red Hat Customer Portal. Режим доступу до ресурсу: https://access.redhat.com/documentation/en-us/red_hat_enterprise_linux/8/html/configuring_infiniband_and_rdma_networks/understanding-infiniband-and-rdma_configuring-infiniband-and-rdma-networks"
            ])
    }
}

trait ReferencesListComponent {
    fn add_references_list_component(self, context: &mut Context, references: &[&str]) -> Self;
}

impl ReferencesListComponent for Docx {
    fn add_references_list_component(self, context: &mut Context, references: &[&str]) -> Self {
        let numbering = context.next_numbering_id();

        let document = self
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
            .add_numbering(Numbering::new(numbering, numbering));

        references.into_iter().fold(document, |document, reference| document.add_paragraph(Paragraph::new()
            .line_spacing(LineSpacing::new().line(24 * 15))
            .numbering(NumberingId::new(numbering), IndentLevel::new(0))
            .align(AlignmentType::Both)
            .add_run(Run::new().add_text(*reference))
        ))
    }
}