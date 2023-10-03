use {
    std::{io::{BufReader, Read}, fs::File, mem, slice},
    tracing::info,
};

// based on this amazing implementation: https://github.com/karpathy/llama2.c/blob/master/run.c

struct Transformer;

#[repr(C, packed)]
#[derive(Debug)]
struct Config {
    dim: i32, // transformer dimension
    hidden_dim: i32, // for ffn layers
    n_layers: i32, // number of layers
    n_heads: i32, // number of query heads
    n_kv_heads: i32, // number of key/value heads (can be < query heads of multiquery)
    vocab_size: i32, // vocabulary size, usually 256 (byte-level)
    seq_len: i32, // max sequence length
}

impl Transformer {
    pub fn build_transformer(&self, checkpoint_path: &str) {
        self.read_checkpoint(checkpoint_path);
        // malloc_run_state is not needed
    }

    pub fn read_checkpoint(&self, checkpoint_path: &str) {
        // TODO: implement reading Config
        let mut config: Config = unsafe { mem::zeroed() };
        let mut buf_reader = BufReader::new(File::open(checkpoint_path).unwrap());

        let config_size = mem::size_of::<Config>();
        unsafe {
            let config_slice = slice::from_raw_parts_mut(&mut config as *mut _ as *mut u8, config_size);
            buf_reader.read_exact(config_slice).unwrap();
        }
        // negative vocab size is hacky way of signaling unshared weights. bit yikes.
        let shared_weights = if config.vocab_size > 0 { 1 } else { 0 };
        config.vocab_size = config.vocab_size.abs();
        // TODO: read model files.

        // TODO: implement remaining
    }
}

pub fn run_llm_inference_demo() {
    info!("running llm inference demo");

    // default parameters
    let checkpoint_path = "./data/stories15M.bin";
    let tokenizer_path = "tokenizer.bin";
    let temperature = 1.0;
    let topp = 0.9;
    let steps = 256;
    let prompt: Option<String> = None;
    let rng_seed = 0;
    // mode is chat
    // system_prompt is none

    // arguments parsing is not performed for simplicity
    // parameter validation/overrides is skipped, because arguments are not provided by user

    // build transformer via the model .bin file
    let transformer = Transformer;
    transformer.build_transformer(&checkpoint_path);
    // TODO: continue implementation
}