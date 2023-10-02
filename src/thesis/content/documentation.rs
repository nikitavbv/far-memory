use crate::thesis::engine::{Block, subsection_header, section_header, paragraph};

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
    - optimize objects placement in spans (keep hot objects together).
    - record stats for spans (access time) - that will allow to perform offline simulations.
      - stat can be access time within a window.
      - swap in/out events.
      - tracking some metrics to know what latency is overall if is SLO is breached or not would also be cool.
        - it is probably possible to track each interaction with smart pointer/swap device.
      - model for eviction.
      - model for prefetching.
      - model for shuffling.
    - fragmentation is a problem to solve later. The simplest way to solve it is size classes.
      - size classes can be tuned by model, by the way.
    */

    Block::Multiple(vec![
        section_header("far memory"),
        Block::Note(r#"Please note that most parts of documentation for this project are in Ukrainian because I am working on this in scope of my thesis at Kyiv Polytechnic Institute and I
need to be able to refer to this documentation when talking to thesis supervisors and other people from the university. I will probably add English translation later."#.to_owned()),
        
        section_header("Віддалена памʼять"),
        paragraph("Віддалена памʼять - тип памʼяті що знаходиться на віддалених вузлах у розподіленій системі."),
        
        section_header("Формалізація задачі"),
        paragraph("Для прикладного програмного забезпечення, в яке інтегрована віддалена памʼять, максимізувати частку даних що зберігається у віддаленій памʼяті при умові дотримання вимог цільового рівня ..."),

        section_header("Підзадачі"),
        paragraph("Нижче наведені підзадачі які потрібно виіршити для реалізації віддаленої памʼяті у порядку їх важливості."),

        subsection_header("Зниження затримки доступу (latency)"),
        paragraph(""),

        subsection_header("Забезпечення відмовостійкості"),

        subsection_header("Інтеграція у існуюче та нове програмне забезпечення"),
    ])
}