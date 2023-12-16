use {
    docx_rs::{Docx, PageMargin, RunFonts, SectionType},
    itertools::Itertools,
    plotters::{prelude::*, coord::{Shift, types::RangedCoordf64}},
    crate::{
        thesis::{
            engine::{Block, ParagraphBlock, TextSpan, SectionHeaderBlock, SubsectionHeaderBlock, ImageBlock, Reference},
            content::{classification_code, keywords, Language},
            utils::mm_to_twentieth_of_a_point,
        },
        demo::evaluation::{load_evaluation_data, Experiment, DemoApplicationType, SpanReplacementPolicy, EvaluationData},
    },
};

const FONT_SIZE: usize = 2 * 12;
const INTERVAL: f32 = 1.15;

pub fn conference_abstract(language: &Language) -> Block {
    let body = conference_abstract_body(language);
    let references = extract_references_text(&body);

    Block::Multiple(vec![
        body,
        Block::SubsectionHeader(
            SubsectionHeaderBlock::without_numbering("References.".to_owned())
                .without_tab()
                .center()
                .bold()
                .with_line_spacing(FONT_SIZE, INTERVAL)
        ),
        Block::OrderedList(references.into_iter().map(|v| TextSpan::Regular(v)).collect()),
        end_section(1)
    ])
}

fn extract_references_text(block: &Block) -> Vec<String> {
    let mut result = Vec::new();
    extract_references_text_inner(&mut result, block);
    result
}

fn extract_references_text_inner(references: &mut Vec<String>, block: &Block) {
    match &block {
        Block::Paragraph(paragraph) => extract_references_text_span(references, paragraph.text()),
        Block::Multiple(multi) => multi.iter().for_each(|v| extract_references_text_inner(references, v)),
        Block::OrderedList(list) => list.iter().for_each(|v| extract_references_text_span(references, v)),
        _ => (),
    }
}

fn extract_references_text_span(references: &mut Vec<String>, text: &TextSpan) {
    match &text {
        TextSpan::Regular(_) => (),
        TextSpan::Bold(inner) => extract_references_text_span(references, inner),
        TextSpan::Italic(inner) => extract_references_text_span(references, inner),
        TextSpan::Multiple(multi) => multi.iter().for_each(|v| extract_references_text_span(references, v)),
        TextSpan::Link { .. } => (),
        TextSpan::Reference(inner, reference) => {
            extract_references_text_span(references, inner);

            let ref_text = reference.text();
            if !references.contains(&ref_text.to_owned()) {
                references.push(ref_text.to_owned());
            }
        },
        TextSpan::Break => (),
    }
}

fn conference_abstract_body(language: &Language) -> Block {
    let ref_far_memory_warehouse_scale = Reference::for_publication(
        "Software-defined far memory in warehouse-scale computers".to_owned(),
        "Andres Lagar-Cavilla, Junwhan Ahn, et al.".to_owned(),
        2019,
        "International Conference on Architectural Support for Programming Languages and Operating Systems".to_owned(),
    );

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
            TextSpan::Break,
            TextSpan::Multiple(vec![
                TextSpan::Italic(Box::new(TextSpan::Multiple(vec![
                    TextSpan::Bold(Box::new(TextSpan::Regular("Holovchenko Maxim Mykolayovych".to_owned()))),
                    TextSpan::Regular(",".to_owned()),
                    TextSpan::Break,
                    TextSpan::Regular("senior lecturer at computer science and software engineering department".to_owned()),
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
        paragraph_without_after_space(TextSpan::Multiple(vec![
            TextSpan::Bold(Box::new("Introduction.".into())),
            " Modern datacenters rely on various approaches to improving resource efficiency, one of which is resource disaggregation. Instead of resource (\
storage devices) being located on individual servers, it is separated into dedicated storage infrastructure that forms a pool of storage shared \
between all compute nodes. This allows to assign as much storage to individual nodes as needed and avoid extra disk space \
to remain unused. ".into(),
            "For random access memory (RAM), operators of world's largest datacenters report average utilization of around ".into(),
            TextSpan::Reference(Box::new(TextSpan::Regular("60%".to_owned())), Reference::for_publication(
                "Borg: the Next Generation".to_owned(),
                "Muhammad Tirmazi, Adam Barker, Nan Deng, et al.".to_owned(),
                2020,
                "Proceedings of ACM EuroSys".to_owned(),
            )),
            " which is mainly caused by individual nodes in datacenters underutilizing RAM provided by hardware. Separating RAM into dedicated infrastructure \
that is accessed over the network is not possible because it causes typical software to significantly degrade in performance, breaching service level objectives \
(SLOs).".into()
        ])),
        paragraph_without_after_space("One approach to solve this is software-defined far memory. This method is based on moving some chunks of data \
nodes with heavy RAM utilization to nodes underutilized RAM and access this data over the network in a way that is \
transparent to the software. This results in higher memory utilization overall while also allowing software to process datasets that are larger in size than \
RAM of a single compute node. The goal of far memory is to move as many data as possible from local memory to remote nodes while solving challenges that \
this configuration introduces. Far memory software should ensure high performance of memory access operations, provide fault tolerance, \
 integrate without significant changes to the codebase while not relying on specialized hardware."),
        end_section(1),

        paragraph_without_after_space(TextSpan::Multiple(vec![
            TextSpan::Bold(Box::new("Overview of existing methods.".into())),
            " There are not many existing works in this field because operators of the largest datacenters became interested in using far memory relatively \
recently. At the time of writing, ".into(),
            TextSpan::Reference(Box::new(TextSpan::Regular("Carbink".to_owned())), Reference::for_publication(
                "Carbink: Fault-tolerant Far Memory".to_owned(),
                "Yang Zhou, Hassan Wassel, Sihang Liu, et al.".to_owned(),
                2022,
                "Proceedings of the 16th USENIX Symposium on Operating Systems Design and Implementation".to_owned(),
            )),
            " is considered a state of the art far memory implementation along with multiple other notable works.".into(),
        ])),
        paragraph_without_after_space("While Carbink is an advanced far memory implementation, it is closed source, tied to the infrastructure and tooling of a \
specific datacenter operator (Google) and is not available for external use. It relies on and supports only application-level integration. \
Memory spans replacements and defragmentation are reduced based on simple heuristics."),
        paragraph_without_after_space(TextSpan::Multiple(vec![
            TextSpan::Reference(Box::new(TextSpan::Regular("AIFM: High-Performance, Application-Integrated Far Memory".to_owned())), Reference::for_publication(
                "AIFM: High-Performance, Application-Integrated Far Memory".to_owned(),
                "Ruan, Zhenyuan and Schwarzkopf, et al.".to_owned(),
                2020,
                "Proceedings of the 14th USENIX Conference on Operating Systems Design and Implementation".to_owned(),
            )),

            " shows the benefit of application-level far memory integration. \
However, this implementation supports only one storage node and does not provide fault tolerance.".into()
        ])),
        paragraph_without_after_space(TextSpan::Multiple(vec![
            "Some methods of providing far memory, like ".into(),

            TextSpan::Reference(Box::new(TextSpan::Regular("Hydra".to_owned())), Reference::for_publication(
                "Hydra : Resilient and Highly Available Remote Memory".to_owned(),
                "Youngmoon Lee, Hasan Al Maruf, et al.".to_owned(),
                2022,
                "20th USENIX Conference on File and Storage Technologies (FAST 22)".to_owned(),
            )),

            ", rely on specialized hardware, for example network interface cards \
supporting Remote Direct Memory Access (RDMA) like InfiniBand. While it allows to transfer spans between nodes with low latency, installing or upgrading hardware \
may not be desirable or achievable in most environments due to costs associated with it that may outweigh the benefits provided by far memory.".into(),
        ])),
        paragraph_without_after_space(TextSpan::Multiple(vec![
            "Other methods, like ".into(),
            TextSpan::Reference(Box::new(TextSpan::Regular("\"Software-Defined Far Memory in Warehouse-Scale Computers\"".to_owned())), ref_far_memory_warehouse_scale.clone()),
            ", use more advanced approaches to \
optimize far memory performance, including statistics collection across the fleet to build a model predicting optimal parameters for the system. However, this \
implementation uses disk as storage backend, which is not optimal for many applications due to lower performance compared to storing data in RAM of remote nodes.".into()
        ])),
        paragraph_without_after_space("These properties and problems of existing solutions create a need for an alternative method of providing far memory."),
        paragraph_without_after_space(TextSpan::Multiple(vec![
            TextSpan::Bold(Box::new("Designing a method and software for providing far memory.".into())),
            " Method of providing far memory that is dicussed in this work, similarly to Carbink, uses following components: compute nodes, \
storage nodes and manager node. \
Far memory client is integrated into compute nodes and works with memory spans (sequences of bytes) that can be located in the local or remote memory. \
Far memory client swaps out memory spans under memory pressure and swaps them back in when access is requested by the software. \
Storage nodes store spans data that were swapped out and function as a key-value storage. Manager node allocates space on storage nodes \
and assigns it for use by specific compute nodes. It also tracks health of all components and restores data on storage nodes that go down as well as provides means \
for scheduled maintenance.".into()])),

        image_with_scale("./images/components.jpg", "Far memory components", 0.55),

        paragraph_without_after_space(TextSpan::Multiple(vec!["To integrate far memory into the software, the method discussed in this work takes two approaches. \
The first one (inspired by Carbink and AIFM) is application-level integration with a far memory client library. Client library works by \
wrapping data managed by far memory into two nested smart pointers (similar to AIFM but second smart pointer is introduced instead of relying on dereference scopes). When the first pointer (FarMemory<T>) is dereferenced, far memory client checks if \
relevant span is located in local or remote memory. Far memory client swaps it in if needed and returns another smart pointer (FarMemoryLocal<T>). When this \
pointer is dereferenced, application receives reference to underlying object (&T) and proceeds to work with it as with any other object \
in RAM. For each span, a reference counter is maintained and increased on each dereference of the first smart pointer. When the second smart pointer goes \
out of scope (implemented by Drop trait in Rust), reference counter is decreased. When it reaches zero, far memory client may swap it out in case of memory \
pressure. When the first pointer goes out of scope, data is removed from local and remote memory because it cannot be accessed by software anymore at this point. \
Taking inspiration from AIFM, client library also provides implementations of data structures designed for use with far memory which are more \
efficient due to additional information \
available during memory access event (for example, knowing which specific part of data structure is accessed allows to swap it in only partially). Unlike AIFM, \
no computation is shifted to storage nodes.
These data \
structures include \
byte buffer, vector, hash table and others. Another important aspect is conversion of objects into byte sequence and vice versa. The simplest approach is just \
copying the whole area of memory where object is stored as is. While far memory client implements this approach, it is not optimal for a number of use cases. \
Because typically data structures contain pointers to other objects that need to be traversed, far memory client provides a wrapper that \
relies on serialization to encode object and nested fields when swapping out.".into()
        ])),
        paragraph_without_after_space(TextSpan::Multiple(vec!["A different method is used to integrate into software without any changes to the codebase. \
Far memory client implements a virtual block device (based on ".into(),
            TextSpan::Reference(Box::new(TextSpan::Regular("NBD".to_owned())), Reference::for_website(
                "Network Block Device - The Linux Kernel documentation".to_owned(),
                "https://docs.kernel.org/admin-guide/blockdev/nbd.html".to_owned()
            )),
            ") that can be used to place \
Linux swap partition on it (similar to the approach used by ".into(),
            TextSpan::Reference(Box::new(TextSpan::Regular("InfiniSwap".to_owned())), Reference::for_publication(
                "Efficient Memory Disaggregation with Infiniswap".to_owned(),
                "Juncheng Gu, Youngmoon Lee, et al.".to_owned(),
                2017,
                "14th USENIX Symposium on Networked Systems Design and Implementation (NSDI 17)".to_owned(),
            )),
            "). This allows to move infrequently accessed memory pages (by swapping mechanisms \
in operating system) to far memory with performance higher than if swapping was performed to disk as it happens normally. This method also allows to use far \
memory as a form of RAM disk.".into()
        ])),
        paragraph_without_after_space(TextSpan::Multiple(vec!["To make far memory more reliable given expanded failure domain, this method of \
providing far memory follows the same approach to this problem as Carbink and uses ".into(),
            TextSpan::Reference(Box::new(TextSpan::Regular("Reed-Solomon".to_owned())), Reference::for_website(
                "An introduction to Reed-Solomon codes: principles, architecture and implementation".to_owned(),
                "https://www.cs.cmu.edu/~guyb/realworld/reedsolomon/reed_solomon_codes.html".to_owned()
            )),
            " coding to compute parity shards for spans and place them on different storage nodes. In the event of node failure this allows to restore data \
using shards from other nodes while keeping recovery time low and using less additional memory compared to replication.".into()
        ])),
        paragraph_without_after_space("Performance is critical for far memory and defines types of software where it can be applied."),

        image_with_scale("./images/fault_tolerance.jpg", "Swapping spans to multiple nodes using Reed-Solomon coding", 0.55),

        paragraph_without_after_space("It is not possible to make far memory as fast as local RAM, however additional latency can be minimzed to \
level that acceptable for real world applications. There is a balance between how actively far memory is used by the application and impact on its performance. \
It is up to application developer how much performance they are willing to trade for lower local memory usage."),
        paragraph_without_after_space("To make far memory performant, the client uses hardware resources efficiently by avoiding unnecessary \
copying of data and communicating with other nodes using lightweight network protocol that is based on TCP. Far memory client implements partial span swap \
out (unlike Carbink) to move as much memory as \
required to maintain enough free memory which is beneficial when dealing with large spans. To avoid blocking application threads with waiting for enough free \
memory on swap in, a background thread is running in a loop swapping out spans with low probability of access (similar to Carbink and AIFM that shift various \
operations to background threads to reduce blocking of the application)."),
        paragraph_without_after_space("However, the key to making far memory performance more close to local RAM is always keeping data that application is about \
to access local. To achieve this, a background thread is picking spans with high probability of access according to replacement algorithm and swap them in in advance. \
In ideal scenario, correct spans are transferred to local memory quickly enough and application will never be blocked by waiting for swap in in main thread."),
        paragraph_without_after_space(TextSpan::Multiple(vec!["It is easy to notice that the algorithm of choosing spans to swap out (and swap in in advance) plays significant role in far \
memory performance. To maximize performance, each time when swap out is needed it is more optimal to pick spans that will be accessed last of all. At the same time, \
for swap in in advance it is better to pick spans that are going to be accessed sooner than other spans. This creates a need for span replacement algorithm that \
takes span access history (including previous application runs) as an input and produces candidates for swap in and swap out. With this formulation, it it similar \
to ".into(),
        TextSpan::Reference(Box::new(TextSpan::Regular("page replacement algorithms".to_owned())), Reference::for_website(
            "Page Replacement Algorithm - Wikipedia.".to_owned(),
            "https://en.wikipedia.org/wiki/Page_replacement_algorithm".to_owned()
        )),
        " in operating systems. Software implementation of this method of providing far memory includes random \
replacement algorithm, least recently used algorithm, most recently used algorithm.".into()
        ])),
        image_with_scale("./images/span_replacement.jpg", "Span replacement algorithm based on memory access statistics", 0.55),
        paragraph_without_after_space(TextSpan::Multiple(vec!["Real world software has different and complex memory access patterns which makes relying on simple \
heurisitics \
inefficient. For example, LRU will not be efficient for software that scans all of its working set sequently in cycle. Unlike existing methods, that rely \
on simple heuristics, this work offers to take a different approach. Specifically, given that there is relatively \
low number of spans in the system, it is feasible to collect and track access statistics for all of them. These stats are sent from compute notes to manager node \
that processes them by building models that predict next span access events for complex span access patterns (this is inspired by ".into(),
        TextSpan::Reference(Box::new(TextSpan::Regular("".to_owned())), ref_far_memory_warehouse_scale),
        ", but instead of optimizing hyperparameters, collected statistics are used to build models that predict which spans will be accessed). \
This model is later used by compute nodes as a span replacement algorithm. This work includes an \"optimal model\" (based on ".into(),

        TextSpan::Reference(Box::new(TextSpan::Regular("Belady's algorithm".to_owned())), Reference::for_website(
           "Page replacement (CS4410) - Cornell University".to_owned(),
           "https://www.cs.cornell.edu/courses/cs4410/2015su/lectures/lec15-replacement.html".to_owned(),
        )),

        ") that picks spans for swap \
operations perfectly given unchanging memory access patterns. For software with dynamic memory access patterns, implementation based on recurrent neural network \
is provided.".into()])),
        paragraph_without_after_space("Thus, the novelty of the proposed method of providing far memory lies in the fact that, unlike existing methods, \
the problem of span replacement is solved statistically more efficiently through the implementation of span access model prediction parameters adaptation \
based on statistics that are continously collected during software runtime."),
        paragraph_without_after_space(TextSpan::Multiple(vec![
            TextSpan::Bold(Box::new("Performance evaluation.".into())),
            " Evaluation of this method of providing far memory seeks to answer the following questions: ".into(),
        ])),
        Block::OrderedList(vec![
           "What end-to-end performance does this method of providing far memory achieve for typical applications with different memory access patterns?".into(),
           "How span access distribution affects performance of far memory operations?".into(),
           "What end-to-end performance is achieved with different span replacement algorithms?".into(),
        ]),
        paragraph_without_after_space(TextSpan::Multiple(vec![
            "To answer these questions a number of experiments were run on two servers with 64GB RAM and 10 \
Gbit/s NIC. To evaluate end-to-end peformance of far memory it was integrated into three synthetic applications with different memory access \
patterns:".into()
        ])),
        Block::OrderedList(vec![
           "Large language model inference application, where weights are stored in far memory. This software represents class of tasks where the whole working set is scanned in pre-defined order.".into(),
           TextSpan::Multiple(vec!["Web service application that accepts zipf-distributed requests".into(),
                " to compute an index to a collection of 8KB objects one of which is picked, encrypted, compressed and sent as response. This software represents a class of software built around \
key-value data structures, where memory access is performed to a lot of small objects with a certain distribution.".into(),
           ]),
           TextSpan::Multiple(vec!["An application that performs queries over a dataframe with data from ".into(),
               TextSpan::Reference(Box::new(TextSpan::Regular("Kaggle delayed flights dataset".to_owned())), Reference::for_website(
                   "Flight Status Prediction - Kaggle".to_owned(),
                   "https://www.kaggle.com/datasets/robikscube/flight-delay-dataset-20182022/".to_owned()
               )),
               ". Dataframe is stored in far memory and is loaded \
row by row as query is processed similarly to typical data processing system or a database. High level data structures provided by far memory client are used
allowing more efficient processing of the stream.".into(),
           ]),
        ]),
        paragraph_without_after_space(TextSpan::Multiple(vec![
            "In each case, far memory client is ran with default settings and system throughput is measured with different levels of local memory ratio.".into(),
        ])),

        paragraph_without_after_space("Based on collected data, it can be noted that this far memory method works best for applications that can utilize high-level \
data structures that are designed for use with far memory. When working with objects, far memory achieves"),

        end_section(2),
        demo_throughput(),
        throughput_distribution(),
        throughput_replacement_policies(),
        end_section(3),

        paragraph_without_after_space("better throughput with larger objects and predictable access patterns. By running web service application \
with different distribution of requests, it can be confirmed that applications \
with high skew of requests distribution can benefit from using far memory while having lower performance impact compared to applications with uniform \
distribution of requests. After running the neural network inference application with different span replacement algorithms, an improvement to \
throughput was observed for span replacement algorithm that relies on span access statistics from previous software runs."),

        end_section(2),
        paragraph_with_before_space(TextSpan::Multiple(vec![
            TextSpan::Bold(Box::new("Conclusion.".into())),
            " This work proposes a method of providing software-defined far memory in distributed systems. Method and software that was designed ensures \
integration simplicity, fault tolerance and high data access performance without relying on specializied hardware. Span replacement algorithm choice was \
analyzed as a factor of far memory performance. Relying on recoding and analyzing span access statistics to build a model for span replacement has shown \
better performance compared to simple heurisitics used by existing approaches to providing far memory.".into(),
        ])),
    ])
}

fn throughput_replacement_policies() -> Block {
    // data
    let evaluation_data = load_evaluation_data();
    let steps = (5..=100).step_by(5).collect::<Vec<_>>();

    fn plot(evaluation_data: &EvaluationData, experiments: &[Experiment]) -> Vec<(f64, u32)> {
        experiments.into_iter()
                .map(|v| (v.local_memory_percent as f64 / 100.0, evaluation_data.get_experiment_result(&v)))
                .filter(|(_, result)| result.is_some())
                .map(|(percent, result)| (percent, result.unwrap() as u32))
                .collect::<Vec<_>>()
    }

    fn normalize(data: &[(f64, u32)], max: u32) -> Vec<(f64, f64)> {
        if data.is_empty() {
            vec![]
        } else {
            data.into_iter()
                .map(|v| (v.0 as f64, v.1 as f64 / max as f64))
                .collect()
        }
    }

    let results_random = plot(
        &evaluation_data,
        &experiments_for_replacement_policy(&steps, SpanReplacementPolicy::Random)
    );

    let results_optimal = plot(
        &evaluation_data,
        &experiments_for_replacement_policy(&steps, SpanReplacementPolicy::Replay)
    );

    let results_lru = plot(
        &evaluation_data,
        &experiments_for_replacement_policy(&steps, SpanReplacementPolicy::LRU)
    );

    let random_max = results_random.iter().map(|v| v.1).max().unwrap();
    let optimal_max = results_optimal.iter().map(|v| v.1).max().unwrap();
    let lru_max = results_lru.iter().map(|v| v.1).max().unwrap();

    let max = random_max.max(optimal_max).max(lru_max);

    let results_random = normalize(&results_random, max);
    let results_optimal = normalize(&results_optimal, max);
    let results_lru = normalize(&results_lru, max);

    // graph
    let k = 20;
    let root_area = BitMapBackend::new("./output/images/replacement_policies.png", (k * 55, k * 45)).into_drawing_area();
    root_area.fill(&WHITE).unwrap();

    let mut cc = setup_chart_context(&root_area);

    cc.configure_mesh()
        .x_labels(10)
        .y_labels(10)
        .axis_style(BLACK.stroke_width(4))
        .disable_mesh()
        .x_label_formatter(&|v| format!("{:.0}%", v * 100.0))
        .y_label_formatter(&|v| format!("{:.1}", v))
        .x_label_style(TextStyle::from(("arial", 48).into_font()))
        .y_label_style(TextStyle::from(("arial", 48).into_font()))
        .x_desc("Local Memory")
        .y_desc("Normalized Throughput")
        .draw()
        .unwrap();

    cc.draw_series(LineSeries::new(
        results_random,
        RED.stroke_width(4)
    )).unwrap().label("random").legend(|(x, y)| PathElement::new(vec![(x, y), (x + 30, y)], RED.stroke_width(4)));

    cc.draw_series(LineSeries::new(
        results_optimal,
        GREEN.stroke_width(4)
    )).unwrap().label("stats-based").legend(|(x, y)| PathElement::new(vec![(x, y), (x + 30, y)], GREEN.stroke_width(4)));

    cc.draw_series(LineSeries::new(
        results_lru,
        BLUE.stroke_width(4)
    )).unwrap().label("LRU").legend(|(x, y)| PathElement::new(vec![(x, y), (x + 30, y)], BLUE.stroke_width(4)));

    cc.configure_series_labels().position(SeriesLabelPosition::Coordinate(20, 10)).legend_area_size(40).margin(10).border_style(BLACK.stroke_width(3)).label_font(("arial", 50).into_font()).draw().unwrap();

    root_area.present().unwrap();

    image_with_scale("./output/images/replacement_policies.png",  "Throughput by replacement algorithm and ratio of local memory", 0.35)
}

fn experiments_for_replacement_policy(steps: &[u32], span_replacement_policy: SpanReplacementPolicy) -> Vec<Experiment> {
    steps
        .iter()
        .map(|local_memory_percent| Experiment {
            local_memory_percent: *local_memory_percent,
            application: DemoApplicationType::LlmInference,
            zipf_s: None,
            span_replacement_policy: Some(span_replacement_policy.clone()),
        })
        .collect::<Vec<_>>()
}

fn throughput_distribution() -> Block {
    // data
    let evaluation_data = load_evaluation_data();
    let experiments = (5..=100).step_by(5)
        .into_iter()
        .map(|zipf_s| Experiment {
            local_memory_percent: 80,
            application: DemoApplicationType::WebService,
            zipf_s: Some(zipf_s),
            span_replacement_policy: None,
        })
        .collect::<Vec<_>>();

    let results = normalize_throughput(&experiments
        .iter()
        .map(|v| (v.zipf_s.unwrap() as f64 / 100.0, evaluation_data.get_experiment_result(&v)))
        .filter(|(_, result)| result.is_some())
        .map(|(percent, result)| (percent, result.unwrap() as u32))
        .collect::<Vec<_>>());

    // graph
    let k = 20;
    let root_area = BitMapBackend::new("./output/images/throughput-distrubution.png", (k * 55, k * 45)).into_drawing_area();
    root_area.fill(&WHITE).unwrap();

    let mut cc = setup_chart_context(&root_area);

    cc.configure_mesh()
        .x_labels(10)
        .y_labels(10)
        .axis_style(BLACK.stroke_width(4))
        .disable_mesh()
        .x_label_formatter(&|v| format!("{:.1}", v))
        .y_label_formatter(&|v| format!("{:.1}", v))
        .x_label_style(TextStyle::from(("arial", 48).into_font()))
        .y_label_style(TextStyle::from(("arial", 48).into_font()))
        .x_desc("Zipf skew parameter (s)")
        .y_desc("Normalized Throughput")
        .draw()
        .unwrap();

    cc.draw_series(LineSeries::new(
        results,
        RED.stroke_width(4)
    )).unwrap();

    root_area.present().unwrap();

    image_with_scale("./output/images/throughput-distrubution.png",  "Web service throughput by skew of requests", 0.35)
}

fn demo_throughput() -> Block {
    // data
    let evaluation_data = load_evaluation_data();
    let begin = 10;
    let llm_inference_results = throughput_plot_for_experiments(&evaluation_data, &(begin..=100).step_by(5)
            .into_iter()
            .map(|local_memory_percent| Experiment {
                local_memory_percent,
                application: DemoApplicationType::LlmInference,
                zipf_s: None,
                span_replacement_policy: None,
            })
            .collect::<Vec<_>>());

    let web_service_results = throughput_plot_for_experiments(&evaluation_data, &(begin..=100).step_by(5)
            .into_iter()
            .map(|local_memory_percent| Experiment {
                local_memory_percent,
                application: DemoApplicationType::WebService,
                zipf_s: None,
                span_replacement_policy: None,
            })
            .collect::<Vec<_>>());

    let dataframe_results = throughput_plot_for_experiments(&evaluation_data, &(begin..=100).step_by(5)
            .into_iter()
            .map(|local_memory_percent| Experiment {
                local_memory_percent,
                application: DemoApplicationType::Dataframe,
                zipf_s: None,
                span_replacement_policy: None,
            })
            .collect::<Vec<_>>());

    // graph
    let k = 20;
    let root_area = BitMapBackend::new("./output/images/demo-throughput.png", (k * 55, k * 45)).into_drawing_area();
    root_area.fill(&WHITE).unwrap();

    let mut cc = setup_chart_context(&root_area);

    cc.configure_mesh()
        .x_labels(10)
        .y_labels(10)
        .axis_style(BLACK.stroke_width(4))
        .disable_mesh()
        .x_label_formatter(&|v| format!("{:.0}%", v * 100.0))
        .y_label_formatter(&|v| format!("{:.1}", v))
        .x_label_style(TextStyle::from(("arial", 48).into_font()))
        .y_label_style(TextStyle::from(("arial", 48).into_font()))
        .x_desc("Local Memory")
        .y_desc("Normalized Throughput")
        .draw()
        .unwrap();

    cc.draw_series(LineSeries::new(
        llm_inference_results,
        RED.stroke_width(4)
    )).unwrap().label("LLM inference").legend(|(x, y)| PathElement::new(vec![(x, y), (x + 30, y)], RED.stroke_width(4)));

    cc.draw_series(LineSeries::new(
        web_service_results,
        BLUE.stroke_width(4)
    )).unwrap().label("web service").legend(|(x, y)| PathElement::new(vec![(x, y), (x + 30, y)], BLUE.stroke_width(4)));

    cc.draw_series(LineSeries::new(
        dataframe_results,
        GREEN.stroke_width(4)
    )).unwrap().label("dataframe").legend(|(x, y)| PathElement::new(vec![(x, y), (x + 30, y)], GREEN.stroke_width(4)));

    cc.configure_series_labels().position(SeriesLabelPosition::Coordinate(20, 10)).legend_area_size(40).margin(10).border_style(BLACK.stroke_width(3)).label_font(("arial", 50).into_font()).draw().unwrap();

    root_area.present().unwrap();

    image_with_scale("./output/images/demo-throughput.png",  "Application throughput by type and ratio of local memory", 0.35)
}

fn setup_chart_context<'a, 'b>(root_area: &'a DrawingArea<BitMapBackend<'b>, Shift>) -> ChartContext<'a, BitMapBackend<'b>, Cartesian2d<RangedCoordf64, RangedCoordf64>> {
    ChartBuilder::on(&root_area)
        .margin_top(60)
        .margin_bottom(30)
        .margin_left(0)
        .margin_right(60)
        .x_label_area_size(110)
        .y_label_area_size(110)
        .build_cartesian_2d(0.0..1.0, 0.0..1.0)
        .unwrap()
}

fn throughput_plot_for_experiments(evaluation_data: &EvaluationData, experiments: &[Experiment]) -> Vec<(f64, f64)> {
    normalize_throughput(&experiments.into_iter()
            .map(|v| (v.local_memory_percent as f64 / 100.0, evaluation_data.get_experiment_result(&v)))
            .filter(|(_, result)| result.is_some())
            .map(|(percent, result)| (percent, result.unwrap() as u32))
            .collect::<Vec<_>>())
}

fn normalize_throughput(data: &[(f64, u32)]) -> Vec<(f64, f64)> {
    if data.is_empty() {
        vec![]
    } else {
        let max_performance = data.iter().map(|v| v.1).max().unwrap();
        data.into_iter()
            .map(|v| (v.0 as f64, v.1 as f64 / max_performance as f64))
            .collect()
    }
}

fn image(path: &str, description: &str) -> Block {
    image_with_scale(path, description, 0.55)
}

fn image_with_scale(path: &str, description: &str, scaling: f32) -> Block {
    Block::Image(ImageBlock::new(path.to_owned(), description.to_owned()).with_scaling(scaling).with_paper_style())
}

fn end_section(columns: usize) -> Block {
    Block::Paragraph(ParagraphBlock::new(TextSpan::Multiple(vec![])).with_tab(false).with_columns(columns))
}

fn paragraph(text: impl Into<TextSpan>) -> Block {
    paragraph_with_params(text, false, true)
}

fn paragraph_with_before_space(text: impl Into<TextSpan>) -> Block {
    paragraph_with_params(text, true, false)
}

fn paragraph_without_after_space(text: impl Into<TextSpan>) -> Block {
    paragraph_with_params(text, false, false)
}

fn paragraph_with_params(text: impl Into<TextSpan>, before_spacing: bool, after_spacing: bool) -> Block {
    let block = ParagraphBlock::new(text.into()).with_tab(false).with_line_spacing(FONT_SIZE, INTERVAL);
    let block = if after_spacing {
        block.with_after_spacing(300)
    } else {
        block
    };
    let block = if before_spacing {
        block.with_before_spacing(300)
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
