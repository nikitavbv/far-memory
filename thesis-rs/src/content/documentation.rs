use crate::engine::Block;

pub fn documentation() -> Block {
    /*
    methods of integration (there does not seem to be other methods other than these two that I have read about):
    - smart pointers - preferred way.
      - it makes sense to follow the same approach as carbink with size classes for objects.
      - it makes sense to optimize objects placement in spans to keep hot objects together.
    - swap device.
      - for implementation, split into arrays (treat them as objects) of N bytes, keep some of them in RAM and some of them remote.

    - record stats for all objects (not blocks).
      - object id.
      - instance id.
      - timestamp.
      - access count within a window.
    - record events when a page is swapped in or moved to remote memory and why.
      - this will allow to optimize models offline.
      - tracking some metrics to know what latency is overall if is SLO is breached or not would also be cool.
        - it is probably possible to track each interaction with smart pointer/swap device.
    */

    Block::Multiple(vec![
        Block::SectionHeader("far memory".to_owned()),
        Block::Note(r#"Please note that most parts of documentation for this project are in Ukrainian because I am working on this in scope of my thesis at Kyiv Polytechnic Institute and I
need to be able to refer to this documentation when talking to thesis supervisors and other people from the university. I will probably add English translation later."#.to_owned()),
        Block::SubsectionHeader("Віддалена памʼять".to_owned()),
        Block::Placeholder(Box::new(Block::Paragraph("Віддалена памʼять - тип памʼяті що знаходиться на віддалених вузлах у розподіленій системі.".to_owned())), "add text explaining what it is".to_owned()),
    ])
}