use crate::thesis::engine::{Block, section_header, subsection_header, paragraph, SubsectionHeaderBlock};

mod components;

pub fn far_memory_method() -> Block {
    Block::Multiple(vec![
        section_header("Методи та засоби надання віддаленої памʼяті"),
        paragraph("Загалом, принцип роботи віддаленої памʼяті полягає в тому, що програмне забезпечення передає у віддалену памʼять дані для зберігання, а коли до цих даних потрібен доступ, то \
реалізація віддаленої памʼяті за запитом від застосунку переміщує дані з інших більш повільних пристроїв зберігання даних у локальну оперативну памʼять. Після переміщення, програмне \
забезпечення працює з даними так само, як і з будь-якими іншими даними у оперативній памʼяті. Після того, як робота з даними закінчена, вони знов переміщуються на зберігання у інший клас памʼяті. \
Саме це надає можливість зменшити використання оперативної памʼяті вузла та обробляти більше даних ніж обʼєм памʼяті прозоро для програмного забезпечення (тобто без значних змін у код, та те, як \
застосунок працює з даними)."),

        components::components(),

        subsection_header("Інтеграція у програмне забезпечення"),
        // tell how library and service running on the end node would be working more specifically. Tell how spans are swapped in and out here. Tell about remotable pointers, buffers, optimized data structures and streaming.
        // tell about ref-counting and identification which spans are not in use. explain how memory limits work. explain how user is supposed to use the swap file and what is done to prevent recursive swap (idk if that is
        // right name for that). tell about traces as a way to monitor the system (if that is a good topic to explain).
        // explained objects serialization.
        // explain why NBD is used for running on the end node. tell that options are a library or a service running on the end node.
        // Tell how span size is being chosen and what effect it has.

        subsection_header("Забезпечення відмовостійкості"),
        // tell about replication to remote nodes and local SSDs and erasure coding. Tell how exactly data will be restored and deleted. Explain that failure domain becomes larger when far memory is used.
        // tell about healthchecks.

        subsection_header("Забезпечення швидкодії віддаленої памʼяті"),
        // tell about optimizing network requests (why TCP (also, why nodelay is used and duplex) is used and not UDP, or http or some kind of existing RPC implementation).
        // tell about reasoning behind partial swap in/swap out. tell why compression is not used. tell why copies should be avoided. tell a bit about size classes.
        // tell about background swap in and swap out threads and how synchronization should be performed.
        // explain what is the key in minimizing latency (keeping all the needed memory locally and moving it quickly) - like explained in the docs.
        // tell that only 3 out of 5 data shards are needed to minimize latency when restoring data.
        // tell about policies to evict and pre-fetch spans (and how those use stats collected, heuristics, FSM, ML models, including RNN). explain why grouping objects in spans is effective. explain why it is important to reduce fragmentation and how it can be
        // achieved. tell about compaction.
        // explain what typical performance numbers are in various environments.
        // tell about page placement algorithms
        // explain how different software accesses memory. Tell how "ideal" policy works. Tell why "least recently used" can be a bad policy in some cases.

        Block::SubsectionHeader(SubsectionHeaderBlock::without_numbering("Висновки до розділу".to_owned())),
        // general conclusions

        // in third section explain that a demo app was implemented to measure how well everything works and the hardware of the test environment.
    ])
}
