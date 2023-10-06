use {
    std::{io::{BufReader, Read, Seek, SeekFrom}, fs::File, mem, slice},
    tracing::info,
};

// based on this amazing implementation: https://github.com/karpathy/llama2.c/blob/master/run.c
// and this fork of it: https://github.com/gaxler/llama2.rs/blob/llama2-rs/llama2-rs/src/main.rs

const CONF_VALS: usize = 7;
const CONF_SIZE: usize = std::mem::size_of::<[i32; CONF_VALS]>();

struct Config {
    dim: usize,
    hidden_dim: usize,
    n_layers: usize,
    n_heads: usize,
    n_kv_heads: usize,
    vocab_size: usize,
    seq_len: usize,
    shared_weights: bool,
}

impl Config {
    fn from_file(path: &str) -> Self {
        let mut model_bin = File::open(path).unwrap();
        let mut buffer = [0; CONF_SIZE];
        model_bin.read_exact(&mut buffer).unwrap();
        let raw_conf = unsafe { mem::transmute::<[u8; CONF_SIZE], [i32; CONF_VALS]>(buffer) };
        let (vocab_size, shared_weights) = if raw_conf[5] < 0 {
            (-raw_conf[5] as usize, true)
        } else {
            (raw_conf[5] as usize, false)
        };

        Self {
            dim: raw_conf[0] as usize,
            hidden_dim: raw_conf[1] as usize,
            n_layers: raw_conf[2] as usize,
            n_heads: raw_conf[3] as usize,
            n_kv_heads: raw_conf[4] as usize,
            vocab_size,
            seq_len: raw_conf[6] as usize,
            shared_weights,
        }
    }
}

struct Vocab {
    bytes: Vec<u8>,
    offsets: Vec<usize>,
}

impl Vocab {
    pub fn from_file(vocab_size: usize, path: &str) -> Self {
        let mut bytes = Vec::<u8>::new();
        let mut offsets = vec![0usize; 1];
        let mut vocab_bin = File::open(path).unwrap();
        let mut len = [0; 4];
        let mut val = [0; 1];
        for vs in 0..vocab_size {
            vocab_bin.read_exact(&mut len).unwrap();
            let l = unsafe { mem::transmute::<[u8; 4], i32>(len) };
            offsets.push(offsets.last().unwrap() + l as usize);
            (0..l).for_each(|_| {
                vocab_bin.read_exact(&mut val).unwrap();
                bytes.extend(val);
            });
        }

        assert_eq!(offsets.len(), vocab_size + 1);

        Self {
            bytes,
            offsets,
        }
    }
}

struct LlamaWeights<Layer, Rms, Emb, Buf> {
    /// (vocab_size, dim)
    embeddings: Emb,
    layers: Vec<Layer>,
    /// (dim,)
    rms_final: Rms,
    /// (seq_len, head_size/2)
    rope_real: Buf,
    /// (seq_len, head_size/2)
    rope_imag: Buf,
    wcls: Option<Emb>,
}

struct LayerWeights<Lin, Rms, Buf> {
    rms_attn: Rms,
    rms_ffn: Rms,
    wq: Lin,
    wk: Lin,
    wv: Lin,
    wo: Lin,
    w1: Lin,
    w2: Lin,
    w3: Lin,
    /// (seq_len, dim)
    k_cache: Buf,
    /// (seq_len, dim)
    v_cache: Buf,
}

type Ty = f32;
type CPULayerFloat = LayerWeights<Vec<Ty>, Vec<Ty>, Vec<Ty>>;
type Llama2CPUFloat = LlamaWeights<CPULayerFloat, Vec<Ty>, Vec<Ty>, Vec<Ty>>;

impl Llama2CPUFloat {
    fn load_weights(cfg: &Config, path: &str) -> Self {
        let (weights, wcls) = load_raw_karpathy(cfg, path);

        unimplemented!()
    }
}

/// Load raw weights from Karpathy's models
fn load_raw_karpathy(cfg: &Config, path: &str) -> ([Vec<Ty>; 13], Option<Vec<Ty>>) {
    let mut model_bin = File::open(path).unwrap();
    
    model_bin.seek(SeekFrom::Start(CONF_SIZE as u64)).unwrap();

    let mut f = |s: usize| _alloc_and_read(&mut model_bin, s);
    let head_size = cfg.dim / cfg.n_heads;
    (
        [
            f(cfg.vocab_size * cfg.dim),
            f(cfg.n_layers * cfg.dim),
            f(cfg.n_layers * cfg.dim * cfg.dim),
            f(cfg.n_layers * cfg.dim * cfg.dim),
            f(cfg.n_layers * cfg.dim * cfg.dim),
            f(cfg.n_layers * cfg.dim * cfg.dim),
            f(cfg.n_layers * cfg.dim),
            f(cfg.n_layers * cfg.dim * cfg.hidden_dim),
            f(cfg.n_layers * cfg.dim * cfg.hidden_dim),
            f(cfg.n_layers * cfg.dim * cfg.hidden_dim),
            f(cfg.dim),
            f(cfg.seq_len * (head_size / 2)),
            f(cfg.seq_len * (head_size / 2)),
        ],
        cfg.shared_weights.then(|| f(cfg.vocab_size * cfg.dim)),
    )
}

fn _alloc_and_read(file: &mut File, size: usize) -> Vec<Ty> {
    unimplemented!()
}

pub fn run_llm_inference_demo() {
    info!("running llm inference demo");

    let model_path = "./data/stories15M.bin";
    let temperature = 0;
    let tokenizer_path = "./data/tokenizer.bin";

    let config = Config::from_file(&model_path);
    let seq_len = config.seq_len;

    let vocab = Vocab::from_file(config.vocab_size, tokenizer_path);
    let mut weights = LlamaWeights::load_weights(&config, &model_path);

    unimplemented!()
}