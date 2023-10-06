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
        let embeddings = weights[0].clone();

        // Go over all layered weights, and make layer chunk out of them
        let mut w_layer_iters = weights[1..10]
            .iter()
            .map(|v| {
                let csize = v.len() / cfg.n_layers;
                v.chunks(csize).map(|l| l.to_vec())
            })
            .collect::<Vec<_>>();

        let layers = (0..cfg.n_layers)
            .map(|l| LayerWeights::<Vec<Ty>, Vec<Ty>, Vec<Ty>> {
                rms_attn: w_layer_iters[0].next().unwrap(),
                wq: w_layer_iters[1].next().unwrap(),
                wk: w_layer_iters[2].next().unwrap(),
                wv: w_layer_iters[3].next().unwrap(),
                wo: w_layer_iters[4].next().unwrap(),
                rms_ffn: w_layer_iters[5].next().unwrap(),
                w1: w_layer_iters[6].next().unwrap(),
                w2: w_layer_iters[7].next().unwrap(),
                w3: w_layer_iters[8].next().unwrap(),
                k_cache: vec![0 as Ty; cfg.seq_len * cfg.dim],
                v_cache: vec![0 as Ty; cfg.seq_len * cfg.dim],
            })
            .collect();

        let rms_final = weights[10].clone();
        let rope_real = weights[11].clone();
        let rope_imag = weights[12].clone();

        Self {
            embeddings,
            layers,
            rms_final,
            rope_real,
            rope_imag,
            wcls,
        }
    }
}

struct ExecutionState<Buffer> {
    /// Shape:(dim,)
    x: Buffer,
    /// Shape:(dim,)
    xb: Buffer,
    /// Shape:(dim,)
    xb2: Buffer,
    /// Shape:(hidden_dim,)
    h1: Buffer,
    /// Shape:(hidden_dim,)
    h2: Buffer,
    /// (dim,): Q, buffers
    q: Buffer,
    /// (dim,): K buffer
    k: Buffer,
    /// (dim,): V buffer
    v: Buffer,
    /// (n_heads, seq_len): Attention Weight Buffer
    att: Buffer,
    /// Logits: (vocab_size, )
    logits: Buffer,
}

/// Helper to simplify buffer init
pub trait DefaultBuffer {
    fn zeros(size: usize) -> Self;
}

impl DefaultBuffer for Vec<Ty> {
    fn zeros(size: usize) -> Self {
        vec![0 as Ty; size]
    }
}

impl <T: DefaultBuffer> ExecutionState<T> {
    fn init(cfg: &Config) -> Self {
        Self {
            x: T::zeros(cfg.dim),
            xb: T::zeros(cfg.dim),
            xb2: T::zeros(cfg.dim),
            h1: T::zeros(cfg.hidden_dim),
            h2: T::zeros(cfg.hidden_dim),
            q: T::zeros(cfg.dim),
            k: T::zeros(cfg.dim),
            v: T::zeros(cfg.dim),
            att: T::zeros(cfg.n_heads * cfg.seq_len),
            logits: T::zeros(cfg.vocab_size),
        }
    }
}

impl EmbeddingTable<Vec<Ty>> for Vec<Ty> {
    fn token_to_resid_stream(&self, token: usize, dst: &mut Vec<Ty>, cfg: &Config) {
        unimplemented!()
    }
}

impl LinearWeight<Vec<Ty>> for Vec<Ty> {
    fn mat_vec(&self, vec: &Vec<Ty>, dst: &mut Vec<Ty>) {
        unimplemented!()
    }
}

impl RMSNormWeight<Vec<Ty>> for Vec<Ty> {
    fn rms_norm(&self, vec: &Vec<Ty>, out: &mut Vec<Ty>) {
        unimplemented!()
    }

    fn inplace_rms_norm(&self, vec: &mut Vec<Ty>) {
        unimplemented!()
    }
}

/// Execute Llama step
pub trait LlamaExecuter<Buffer> {
    fn step(&mut self, token: usize, pos: usize, cfg: &Config, state: &mut ExecutionState<Buffer>);
}

/// Execute Llama layer
pub trait LlamaLayer<Buffer> {
    /// RMS norm residual stream and get Q,K,V matrices
    fn rms_and_qkv(&self, config: &Config, state: &mut ExecutionState<Buffer>);
    /// Rotate q and k heads according to position in seq (RoPE)
    fn rope(&self, pos: usize, cfg: &Config, state: &mut ExecutionState<Buffer>, rope_imag: &Buffer, rope_real: &Buffer);
    /// Cache sequence of Q, K (to be used for attention computation)
    fn cache_kv(&mut self, pos: usize, cfg: &Config, state: &ExecutionState<Buffer>);
    /// (per head) Calculate Attention weights, accumulate value according to weights
    fn attention(&self, pos: usize, cfg: &Config, state: &ExecutionState<Buffer>);
    /// Merge all heads and add result to residula stream
    fn merge_heads_to_resid_stream(&self, state: &mut ExecutionState<Buffer>);
    /// RMS norm residual stream,
    /// apply FeedForward to normalized
    /// add to residual stream
    fn ffn(&self, state: &mut ExecutionState<Buffer>);
}

pub trait LinearWeight<T> {
    fn mat_vec(&self, vec: &T, dst: &mut T);
}

pub trait RMSNormWeight<T> {
    fn rms_norm(&self, vec: &T, out: &mut T);
    fn inplace_rms_norm(&self, vec: &mut T);
}

pub trait EmbeddingTable<Buf>: LinearWeight<Buf> {
    fn token_to_resid_stream(&self, token: usize, dst: &mut Buf, cfg: &Config);
}

// f32 CPU implementation of Llama2
impl<L, Rms, Emb> LlamaExecuter<Vec<Ty>> for LlamaWeights<L, Rms, Emb, Vec<Ty>>
where
    L: LlamaLayer<Vec<Ty>>,
    Rms: RMSNormWeight<Vec<Ty>>,
    Emb: EmbeddingTable<Vec<Ty>>,
{
    fn step(&mut self, token: usize, pos: usize, cfg: &Config, state: &mut ExecutionState<Vec<Ty>>) {
        unimplemented!()
    }
}

// f32 Implementation of Llama2 layer
impl<Lin, Rms> LlamaLayer<Vec<Ty>> for LayerWeights<Lin, Rms, Vec<Ty>>
where
    Lin: LinearWeight<Vec<Ty>>,
    Rms: RMSNormWeight<Vec<Ty>>,
{
    fn rms_and_qkv(&self, config: &Config, state: &mut ExecutionState<Vec<Ty>>) {
        unimplemented!()
    }

    fn rope(&self, pos: usize, cfg: &Config, state: &mut ExecutionState<Vec<Ty>>, rope_imag: &Vec<Ty>, rope_real: &Vec<Ty>) {
        unimplemented!()
    }

    fn cache_kv(&mut self, pos: usize, cfg: &Config, state: &ExecutionState<Vec<Ty>>) {
        unimplemented!()
    }

    fn attention(&self, pos: usize, cfg: &Config, state: &ExecutionState<Vec<Ty>>) {
        unimplemented!()
    }

    fn merge_heads_to_resid_stream(&self, state: &mut ExecutionState<Vec<Ty>>) {
        unimplemented!()
    }

    fn ffn(&self, state: &mut ExecutionState<Vec<Ty>>) {
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
    let bytes_to_read = size * std::mem::size_of::<Ty>();
    let mut raw_w_data = vec![0; bytes_to_read];
    file.read_exact(&mut raw_w_data).unwrap();
    unsafe {
        let float_ptr = raw_w_data.as_ptr() as *const Ty;
        let data = std::slice::from_raw_parts(float_ptr, size);
        data.to_vec()
    }
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

    let mut state = ExecutionState::<Vec<Ty>>::init(&config);
    let mut probs = vec![0 as Ty; config.vocab_size];

    let mut pos = 0;
    let mut token = 1;
    while pos < seq_len {
        weights.step(token, pos, &config, &mut state);
        unimplemented!()
    }

    unimplemented!()
}