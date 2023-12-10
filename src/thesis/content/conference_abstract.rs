use {
    docx_rs::{Docx, PageMargin, RunFonts, SectionType},
    itertools::Itertools,
    crate::thesis::{
        engine::{Block, ParagraphBlock, TextSpan, SectionHeaderBlock, SubsectionHeaderBlock},
        content::{classification_code, keywords, Language},
        utils::mm_to_twentieth_of_a_point,
    },
};

const FONT_SIZE: usize = 2 * 12;
const INTERVAL: f32 = 1.15;

pub fn conference_abstract() -> Block {
    Block::Multiple(vec![
        paragraph(TextSpan::Multiple(vec![
            format!("UDC {}", classification_code()).into(),
        ])),
        paragraph(TextSpan::Multiple(vec![
            TextSpan::Italic(Box::new(TextSpan::Multiple(vec![
                TextSpan::Bold(Box::new(TextSpan::Regular("Volobuiev Nikita Oleksandrovich".to_owned()))),
                TextSpan::Regular(", master's degree student".to_owned()),
                TextSpan::Break,
                TextSpan::Regular("National Technical University of Ukraine «Igor Sikorsky Kyiv Polytechnic Institute», Ukraine".to_owned()),
            ]))),
            TextSpan::Break,
            TextSpan::Multiple(vec![
                TextSpan::Italic(Box::new(TextSpan::Multiple(vec![
                    TextSpan::Bold(Box::new(TextSpan::Regular("Supervisor: Pavlov Oleksandr Anatoliyovych".to_owned()))),
                    TextSpan::Regular(", doctor of technical sciences,".to_owned()),
                    TextSpan::Break,
                    TextSpan::Regular("professor, professor of computer science and software engineering department".to_owned()),
                    TextSpan::Break,
                    TextSpan::Regular("National Technical University of Ukraine «Igor Sikorsky Kyiv Polytechnic Institute», Ukraine".to_owned()),
                ]))),
            ]),
        ])),
        Block::SectionHeader(
            SectionHeaderBlock::without_numbering("Methods and software for providing software-defined far memory in distributed systems".to_uppercase())
                .do_not_include_in_table_of_contents()
                .without_page_break_before()
        ),
        paragraph(TextSpan::Multiple(vec![
            TextSpan::Bold(Box::new(TextSpan::Regular("Анотація.".to_owned()))),
            " У роботі розглянуто проблему надання віддаленої памʼяті у розподілених системах. Розглянуто підходи до інтеграції віддаленої памʼяті у програмне \
забезпечення, забезпечення відмовостійкості та високого рівня швидкодії. Запропоновано удосконалений алгоритм заміщення проміжків, що спирається на статистику \
доступів до проміжків памʼяті для більш ефективного переміщення проміжків між локальною та віддаленою памʼяттю, що дозволяє знизити затримку доступу до даних \
у порівнянні з більш простими підходами заміщення проміжків.".into(),
            TextSpan::Break,
            TextSpan::Bold(Box::new("КЛЮЧОВІ СЛОВА:".into())),
            " ".into(),
            TextSpan::Multiple(keywords().into_iter().map(|v| v.for_language(&Language::Ukrainian).into()).intersperse(", ".into()).collect()),
            ".".into(),
        ])),
        paragraph(TextSpan::Multiple(vec![
            TextSpan::Bold(Box::new("Abstract.".into())),
            " This work discusses the problem of providing far memory in distributed systems. The approaches to integrating far memory into software, ensuring \
fault tolerance and high performance are reviewed. An improved span replacement algorithm is proposed which is based on memory span access statistics and provides \
more efficient swapping between local and remote memory. This results in reduction of data access latency compared to simple span replacement approaches.".into(),
            TextSpan::Break,
            TextSpan::Bold(Box::new("KEYWORDS:".into())),
            " ".into(),
            TextSpan::Multiple(keywords().into_iter().map(|v| v.for_language(&Language::English).into()).intersperse(", ".into()).collect()),
            ".".into(),
        ])),
        end_section(1),
        paragraph_without_after_space(TextSpan::Multiple(vec![
            TextSpan::Bold(Box::new("Introduction.".into())),
            " Modern datacenters rely on various approaches to improving resource efficiency. For instance, CPU oversubscription is frequently used to improve \
CPU compute time utilization. Another resource is persistent storage for which resource disaggregation is applied in modern infrastructure. Instead of storage \
devices being located on individual compute nodes (servers), it is separated into dedicated storage infrastructure which forms a pool of storage shared \
between all compute nodes. In this configuration, access to data stored on drives is provided over the network. This allows to assign as much storage to compute \
nodes as it is needed by the software running on them. This also avoids situation where storage space on individual nodes remains unused because tasks running \
have lower storage requests than what is provided by hardware.".into(),
        ])),
        paragraph_without_after_space(
            "For random access memory (RAM), operators of world's largest datacenters report average utilization of around 60%. Just as with storage, \
some compute nodes in the cluster may be running software that requires less memory than what the hardware provides. Efficiency of task scheduling is unrelated \
to this problem, because compute nodes may be constrained by some other resource (for example, CPU compute time). Following the exact same approach with RAM as \
with persistent storage is problematic due to more strict performance requirements set for this class of memory. Separating RAM into dedicated infrastructure \
that is accessed over the network significantly affects latency and bandwidth numbers for memory access operations. This difference is enough for typical software \
running on compute nodes to noticably degrade in peformance, breaching service level objectives (SLOs) defined for this software."),
        paragraph_without_after_space("One approach to solve this is software-defined far memory. The idea behind this method is that some chunks of data can be \
moved from compute nodes with heavy RAM utilization to nodes with a lot of free RAM and access this data over the network in a way that is \
transparent to the software (working with data in far memory should be similar to working with data in regular RAM). This results in higher memory utilization \
overall while also allowing software to process datasets that are larger in size than RAM of a single compute node."),
        paragraph_without_after_space("The goal of far memory is to move as many data as possible from local memory to remote nodes while solving challenges that \
this configuration introduces. Far memory implementation should ensure high performance of memory access operations, provide fault tolerance, \
 integrate without significant changes to the codebase while not relying on additional hardware."),
        paragraph_without_after_space(TextSpan::Multiple(vec![
            TextSpan::Bold(Box::new("Overview of existing implementations.".into())),
            " There are not many existing implementations of far memory because this topic became interesting for operators of the largest datacenters relatively \
recently. At the time of writing, Carbink is considered a state of the art far memory implementation along with multiple other notable implementations.".into(),
        ])),
        paragraph_without_after_space("While Carbink is an advanced far memory implementation, it is closed source, tied to the infrastructure and tooling of a \
specific datacenter operator (Google) and is not available for external use. It relies on application-level integration and does not have a way to integrate \
into software be other means. Memory spans replacements and defragmentation is optimized based on simple heuristics that do not rely on analyzing data access \
patterns."),
        paragraph_without_after_space("AIFM: High-Performance, Application-Integrated Far Memory shows the benefit of application-level far memory integration. \
However, this implementation supports only one storage node and does not provide fault tolerance."),
        paragraph_without_after_space("Some implementations, like Hydra, rely on specialized hardware, for example network interface cards \
supporting Remote Direct Memory Access (RDMA) like InfiniBand. While it allows to transfer spans between nodes with low latency, installing or upgrading hardware \
may not be desirable or achievable in most environments. Performing changes to the hardware configuration usually has costs associated with it that \
may outweigh the benefits provided by far memory."),
        paragraph_without_after_space("Other implementations, like \"Software-Defined Far Memory in Warehouse-Scale Computers\", use more advanced approaches to \
optimize far memory performance, including statistics collection across the fleet to build a model predicting optimal parameters for the system. However, this \
implementation uses disk as storage backend, which is not optimal for many applications due to lower performance compared to storing data in RAM of remote nodes."),
        paragraph_without_after_space("These properties and problems of existing solutions create a need for far memory implementation that \
would be open source, integrate into software with little changes to the codebase, while providing fault tolerance and high memory access operations \
performance provided by more efficient span replacement algorithms."),
        paragraph_without_after_space(TextSpan::Multiple(vec![
            TextSpan::Bold(Box::new("Designing a method and software for providing far memory.".into())),
            " The implementation of far memory that is being discussed in this work operates on a similar principle: \
far memory client is integrated into the software, chunks of data managed by it (called spans, represented as byte sequences) are moved to the memory of remote \
nodes to free local memory. Spans are moved back to local memory (swap in) when access to data is requested. Only part of spans being present locally at \
once allow to lower memory usage.".into(),
        ])),
        paragraph_without_after_space("This implementation consists of the following components: compute nodes, storage nodes and manager node. Compute node \
is a node that puts memory spans into the system for storage. Compute nodes may be repesented by different applications and different versions of these \
applications. Spans are the central entity which far memory client operates on and are identified by an ID (64 bit number). Data associated with span may be located \
in the local memory or on storage backend. Far memory implementation that is being discussed in this work supports multiple storage backend implementations: \
local memory, solid state drives, one or multiple remote nodes. The latter is the main mode of operation, while others are implemented for testing purposes \
and to fit specific environments when those storage backends may be more practical. Storage nodes serve the function of storing spans data that were swapped out \
and can be viewed as a key-value storage. Having multiple compute and storage nodes creates a need for a manager node. Manager node allocates space on storage nodes \
and assigns it for use by specific compute nodes. It also tracks health of all components and restores data on storage nodes that go down as well as provides means \
for scheduled maintenance. Additionally, manager node collects span access statistics and uses it to update parameters of the system."),
        paragraph_without_after_space("Integration of far memory into software is a complex problem because modern programming languages are built with an \
assumption that all data is located in local RAM. That makes it difficult to place objects in far memory transparently, because there is no way to create a \
pointer to a different storage device. While operating systems have a concept of virtual memory and memory mapping mechanisms, that cannot be used to provide \
far memory without significant changes into the codebase while providing high performance. For these reasons, the implementation of far memory discussed in this \
work picks two approaches for far memory integration. The first one is application-level integration with a far memory client library. In short, it works by \
creating wrappers for data managed by far memory. Two nested smart pointers are used to track when software requests access to data being located in far memory and \
to identify when it is no longer needed and can be swapped out safely. Far memory client library is written in Rust and supports in-depth configuration of storage \
backend, swap in and swap out processes. Given that providing higher level abstractions allows to make far memory more efficient due to additional information \
available during memory access event (for example, knowing which specific part of data structure is accessed allows to swap it in only partially, avoiding full \
swap in that would happen otherwise) this library provides implementations of data structures optimized for use with far memory. These data structures include \
byte buffer, vector, hash table and others. Another important aspect is conversion of objects into byte sequence and vice versa. The simplest approach is just \
copying the whole area of memory where object is stored as is. While far memory client implements this approach, it is not optimal for a number of use cases. \
Typically, data structures contain pointers to other nested data structures meaning that during swap out (and swap in as well) it may be desirable for client to \
traverse the whole structure and send it to remote node along with nested objects. For this reason, far memory client provides FarMemorySerialized<T> which \
relies on serialization (implemented using serde) to serialize and deserialize object with nested fields when performing swap out."),
        paragraph_without_after_space("Given that scenarios when changing source code of software is not possible exist, this far memory implementation provides \
a different method of integration for such cases. By implementing a virtual block device (based on nbd), far memory client provides a way for the user to place \
Linux swap partition on block device backed by far memory. This allows to move infrequently accessed memory pages (by utilizing existing swapping mechanisms \
in operating system) to far memory with performance higher than if swapping was performed to disk as it happens normally. This method also allows to use far \
memory as a form of RAM disk which may be useful for some types of software."),
        paragraph_without_after_space("Another important aspect of far memory implementation is providing fault tolerance. Moving data to other devices (including \
remote nodes) expands failure domain of the system. It is not possible to make probability of data loss for far memory to be as low as it is for local RAM, but this \
probability can be minimized. While storing a copy of data on disk is supported by this implementation, it is not optimal due to high recovery time and increased \
use of a different storage class (SSD disk space). Another option is data replication to multiple remote nodes, but it results in inefficient use remote nodes \
memory. The most efficient approach is using Reed-Solomon coding which is frequently applied to this class of tasks. In short, when swapping out data is split \
into N shards and additional M parity shards. These shards are stored on different nodes and in the event of node failure and loss of any M shards, data can \
still be restored by performing a linear transformation from the existing shards."),
        paragraph_without_after_space("Performance is critical for far memory and defines field of software and use-cases where far memory can be applied. Data access \
time for data in far memory will always be higher compared to data stored in local RAM because latency and bandwidth numbers for remote storage devices is \
significantly higher than for RAM. In these conditions it is not possible to make far memory as fast as local RAM, however additional latency can be minimzed to \
level that acceptable for real world applications. There is a balance between how actively far memory is used by the application and impact on its performance. \
It is up to application developer how much performance they are willing to trade for lower local memory usage."),
        paragraph_without_after_space("To make this implementation of far memory performant, the client uses hardware resources efficiently by avoiding unnecessary \
copying of data and communicating with other nodes using lightweight network protocol that is based on TCP and uses the simplest form of request serialization \
based on bincode. Compression is not used (but can be optionally enabled by the user) because modern compression algorithms are typically slower (6.4Gbps for lz4) \
than modern network transfer speed (10Gbps and more is typical for datacenter). Far memory client implements partial span swap out to move as much memory as \
required to maintain enough free memory which is beneficial when dealing with large spans. To avoid blocking application threads with waiting for enough free \
memory on swap in, a background thread is implemented to free memory (by swapping out) proactively."),
        paragraph_without_after_space("However, the key to making far memory performance more close to local RAM is always keeping data that application is about \
to access local. One way to achieve this is to swap in spans in advance in a background thread. In ideal scenario, when this background thread chooses spans to \
swap in accurately enough and transfers them to local memory quickly enough, application threads will never be blocked by waiting for far memory client to finish \
swap in of spans. Far memory implementation that is discussed in this paper includes such background thread."),
        paragraph_without_after_space("It is easy to notice that the method of choosing spans to swap out (and swap in in advance) plays significant role in far \
memory performance. To maximize performance, each time when swap out is needed it is more optimal to pick spans that will be accessed last of all. At the same time,
for swap in in advance it is better to pick spans that are going to be accessed sooner than other spans. This creates a need for span replacement algorithm that \
takes span access history (including previous application runs) as an input and produces candidates for swap in and swap out. With this formulation, it it similar \
to page replacement algorithms in operating systems. There are various kinds of span replacement algorithms that can be used for far memory and in this work \
multiple are implemented and can be choosed by user according to their needs. This implementation includes random replacement policy, least recently used policy, \
most recently used policy. Most existing far memory implementations rely on simple heurisitics and algorithms as their replacement policy (usually \"least recently \
used\" policy is used)."),
        paragraph_without_after_space("However, real world software has different and complex memory access patterns which makes relying on simple heurisitic \
inefficient. Imagine software that scans all of its working set sequently in cycle. LRU policy which is popular is actually the least efficient here: it will pick \
exactly those spans for swap out that will be accessed soon. That's why this far memory implementation takes a different approach. Given that there is relatively \
low number of spans in the system, it is feasible to collect and track access statistics for all of them. These stats are sent from compute notes to manager node \
that processes them by building models that can rely on complex span access patterns to better predict next span access events. \
This model is later used by compute nodes used as a span replacement policy. This work includes an \"ideal model\" that picks spans for swap \
operations perfectly given static memory access patterns. For software with dynamic memory access patterns, implementation based on recurrent neural network \
is provided."),
        paragraph_without_after_space(TextSpan::Multiple(vec![
            TextSpan::Bold(Box::new("Performance evaluation.".into())),
            " Evaluation of this far memory implementation seeks to answer the following questions: ".into(),
        ])),
        Block::OrderedList(vec![
           "What end-to-end performance does this far memory implementation achieve for typical applications with different memory access patterns?".to_owned(),
           "How span access distribution affects performance of far memory operations?".to_owned(),
           "What end-to-end performance is achieved with different span replacement policies?".to_owned(),
        ]),
        paragraph_without_after_space(TextSpan::Multiple(vec![
            "To answer these questions a number of experiments were run on two nodes with AMD Ryzen 5 3600 6-core CPUs (3.6GHz), 64GB RAM and Intel 82599 10 \
Gbit/s NIC (direct connectivity). Both nodes are running ArchLinux (with kernel version 6.5.8).".into(),
        ])),
        paragraph_without_after_space(TextSpan::Multiple(vec![
            "To evaluate end-to-end peformance of this far memory implementation it was integrated into three synthetic applications with different memory access \
patterns:".into()
        ])),
        Block::OrderedList(vec![
           "Large language model inference application. Neural network weights are stored in far memory and are fully scanned in interations as text is being \
generated. This software represents class of tasks where the whole working set is scanned in pre-defined order.".to_owned(),
           "Web service application that accepts requests with zipf-distributed user IDs to compute an index (also zipf-distributed) to a collection of pictures \
item from which is picked, encrypted with AES GCM, compressed with Snappy and sent back to the client. This software represents a class of software built around \
key-value data structures, where memory access is performed to a lot of small objects with a certain distribution.".to_owned(),
           "An application that performs queries over a dataframe with data from Kaggle delayed flights dataset. Dataframe is stored in far memory and is loaded \
row by row as query is processed similarly to typical data processing system or a database. In this case, data access pattern is a bit different because \
rows can be processed in any order in a stream which allows far memory client to rely on various optimizations when high level data structures are used.".to_owned(),
        ]),
        paragraph_without_after_space(TextSpan::Multiple(vec![
            "In each case, far memory client is ran with default settings and system throughput is measured with different levels of spans swapped out to remote \
node.".into(),
        ])),

        /* todo: picture with througput for different applications. */
        /* todo: analysis of data. */

        paragraph_without_after_space("When data in far memory is accessed in random order (as in the second demo application), distribution of span access \
plays an important role. When distribution is skewed, far memory client should be able to make data access latency close to RAM access latency because hot \
data should fit local memory. On the other hand, when distribution is more uniform, swap out and swap in performance defines end-to-end performance of the \
system."),
        paragraph_without_after_space("To evaluate that, web service application is run with different s parameters of zipf distribution and throughput is \
measured."),

        /* todo: picture with throughput for different s-param. */
        /* todo: analysis of data. */

        paragraph_without_after_space("Span replacement policy affects how frequently spans will be swapped in from memory of remote nodes blocking execution \
of the application. To evaluate how well different replacement policies perform, throughput was measured for neural network inference application with different \
replacement algorithms and levels of local memory."),

        /* todo: picture with throughput for different replacement policies. */
        /* todo: analysis of data. */

        end_section(2),
        paragraph(TextSpan::Multiple(vec![
            TextSpan::Bold(Box::new("Conclusion.".into())),
            " This work proposes a method of providing software-defined far memory in distributed systems. Method and software that was designed ensures \
integration simplicity, fault tolerance and high data access performance without relying on specializied hardware. ....".into(),
        ])),
        Block::SubsectionHeader(
            SubsectionHeaderBlock::without_numbering("References.".to_owned())
                .without_tab()
                .center()
                .bold()
                .with_line_spacing(FONT_SIZE, INTERVAL)
        ),
        Block::OrderedList(vec![
            "Виконання основних арифметичних дій з комплексними числами, які представлено в інтервальній гіперболічній формі / С. В. Гадецька [та ін.] // Сучасні інформаційні системи = Advanced Information Systems. – 2022. – Т. 6, № 1. – С. 104-113.".to_owned(),
        ]),
        end_section(1)
    ])
}

fn end_section(columns: usize) -> Block {
    Block::Paragraph(ParagraphBlock::new(TextSpan::Multiple(vec![])).with_tab(false).with_columns(columns))
}

fn paragraph(text: impl Into<TextSpan>) -> Block {
    paragraph_with_params(text, true)
}

fn paragraph_without_after_space(text: impl Into<TextSpan>) -> Block {
    paragraph_with_params(text, false)
}

fn paragraph_with_params(text: impl Into<TextSpan>, after_spacing: bool) -> Block {
    let block = ParagraphBlock::new(text.into()).with_tab(false).with_line_spacing(FONT_SIZE, INTERVAL);
    let block = if after_spacing {
        block.with_after_spacing(300)
    } else {
        block
    };
    Block::Paragraph(block)
}

pub fn conference_abstract_docx_template() -> Docx {
    // requirements: https://docs.google.com/document/d/1CoIPOtUko0ZpV3JgNn9JhV-l_kZDbKO8v66zrIS9dzg/edit
    let mut docx = Docx::new()
        .page_margin(
            PageMargin::new()
                .top(mm_to_twentieth_of_a_point(15.0))
                .bottom(mm_to_twentieth_of_a_point(15.0))
                .left(mm_to_twentieth_of_a_point(20.0))
                .right(mm_to_twentieth_of_a_point(20.0))
        )
        .default_fonts(RunFonts::new().cs("Times New Roman"))
        .default_size(FONT_SIZE)
        .default_tab_stop(0);

    docx.document.section_property.section_type = Some(SectionType::Continuous);

    docx
}
