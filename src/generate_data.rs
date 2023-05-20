use {
    std::{fs::File, io::{BufRead, BufReader, Write}},
    indicatif::ProgressIterator,
    rust_bert::pipelines::sentence_embeddings::{
        SentenceEmbeddingsBuilder,
        SentenceEmbeddingsModelType,
    },
};

pub fn generate_data() {
    println!("generating test data");

    let model = SentenceEmbeddingsBuilder::remote(
        SentenceEmbeddingsModelType::AllMiniLmL12V2
    ).create_model().unwrap();

    let input_file = File::open("./data/commoncrawl/data.warc.wet").unwrap();
    let lines: Vec<String> = BufReader::new(&input_file).lines().map(|line| line.unwrap()).collect();
    println!("processing lines");

    let mut vectors = File::create("./data/commoncrawl/vectors").unwrap();

    for line in lines.into_iter().progress() {
        let output = model.encode(&[&line]).unwrap();
        let output = output.get(0).unwrap();
        let encoded = bincode::serialize(&output).unwrap();

        if encoded.len() != 1544 {
            panic!("unexpected encoded len for sentence: {}", line);
        }

        vectors.write(&encoded).unwrap();
    }
}