use {
    std::{io::{self, Read, Seek, SeekFrom, Write}, fs::File, mem, time::Instant, sync::Arc},
    tracing::{info, warn},
    rand::{rngs::SmallRng, SeedableRng, Rng},
    quantiles::ckms::CKMS,
    prometheus::{Registry, register_gauge_with_registry, register_int_counter_with_registry},
    crate::{
        utils::allocator::current_memory_usage,
        manager::ManagerClient,
        client::{
            FarMemoryBuffer,
            FarMemoryClient,
            NetworkNodeBackend,
            LocalDiskBackend,
            FarMemoryBackend,
            FarMemoryBufferedVec,
            FarMemoryVec,
            ReplicationBackend,
            ErasureCodingBackend,
            InstrumentedBackend,
            TrackingReplacementPolicy,
            MostRecentlyUsedReplacementPolicy,
            ReplayReplacementPolicy,
            PreferRemoteSpansReplacementPolicy,
            RemoteReplayReplacementPolicy,
            ReplacementPolicy,
        },
    },
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
    bytes: FarMemoryBuffer,
    offsets: FarMemoryBufferedVec<usize>,
}

impl Vocab {
    pub fn from_file(client: FarMemoryClient, vocab_size: usize, path: &str) -> Self {
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

        let bytes = FarMemoryBuffer::from_bytes(client.clone(), bytes);

        Self {
            bytes,
            offsets: FarMemoryBufferedVec::from_vec(client, offsets),
        }
    }

    fn get_token(&self, idx: usize) -> String {
        let (st, en) = (self.offsets.get(idx), self.offsets.get(idx + 1));
        let b = self.bytes.slice(st..en);
        String::from_utf8(b).unwrap()
    }
}

struct LlamaWeights<Layer> {
    /// (vocab_size, dim)
    // embeddings: Emb,
    embeddings_far: Vec<Ty>,

    layers: Vec<Layer>,
    /// (dim,)
    rms_final: Vec<Ty>,
    /// (seq_len, head_size/2)
    rope_real: Vec<Ty>,
    /// (seq_len, head_size/2)
    rope_imag: Vec<Ty>,
    wcls: Option<Vec<Ty>>,
}

struct LayerWeights<Buf> {
    // rms_attn: Rms,
    rms_attn: Vec<Ty>,

    rms_ffn: Vec<Ty>,
    wq: Vec<Ty>,
    wk: Vec<Ty>,
    wv: Vec<Ty>,
    wo: Vec<Ty>,
    w1: Vec<Ty>,
    w2: Vec<Ty>,
    w3: Vec<Ty>,
    /// (seq_len, dim)
    k_cache: Buf,
    /// (seq_len, dim)
    v_cache: Buf,
}

type Ty = f32;
type CPULayerFloat = LayerWeights<Vec<Ty>>;
type Llama2CPUFloat = LlamaWeights<CPULayerFloat>;

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
            .map(|l| LayerWeights::<Vec<Ty>> {
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
        let rope_real =  weights[11].clone();
        let rope_imag = weights[12].clone();

        Self {
            embeddings_far: embeddings.clone(),

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
    fn token_to_resid_stream(&self, pos: usize, dst: &mut Vec<Ty>, cfg: &Config) {
        let dim = dst.len();
        self.chunks_exact(dim)
            .skip(pos)
            .take(1)
            .for_each(|src| dst.as_mut_slice().copy_from_slice(src));
    }
}

impl LinearWeight<Vec<Ty>> for Vec<Ty> {
    fn mat_vec(&self, vec: &Vec<Ty>, dst: &mut Vec<Ty>) {
        matmul(dst, vec, &self); // in_dim is inferred from x. need to remove from this function dig
    }
}

impl RMSNormWeight<Vec<Ty>> for Vec<Ty> {
    fn rms_norm(&self, vec: &Vec<Ty>, out: &mut Vec<Ty>) {
        let inv_denom = _norm_const(vec);

        let w_it = self.iter();
        let normed = vec.iter().zip(w_it).map(|(xx, ww)| xx * ww * inv_denom);
        out.iter_mut().zip(normed).for_each(|(dst, src)| *dst = src);
    }

    fn inplace_rms_norm(&self, vec: &mut Vec<Ty>) {
        let inv_denom = _norm_const(vec);

        let w_it = self.iter();
        vec.iter_mut()
            .zip(w_it)
            .for_each(|(dst, w)| (*dst) *= inv_denom * w);
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
impl<L> LlamaExecuter<Vec<Ty>> for LlamaWeights<L>
where
    L: LlamaLayer<Vec<Ty>>,
{
    fn step(&mut self, token: usize, pos: usize, cfg: &Config, state: &mut ExecutionState<Vec<Ty>>) {
        // copy token embedding to residual stream
        self.embeddings_far.to_local_vec().token_to_resid_stream(token, &mut state.x, cfg);

        for ld in self.layers.iter_mut() {
            ld.rms_and_qkv(cfg, state);
            ld.rope(pos, cfg, state, &self.rope_imag.to_local_vec(), &self.rope_real.to_local_vec());
            ld.cache_kv(pos, cfg, state);
            ld.attention(pos, cfg, state);
            ld.merge_heads_to_resid_stream(state);
            ld.ffn(state);
        }

        self.rms_final.to_local_vec().inplace_rms_norm(&mut state.x);

        if self.wcls.is_none() {
            self.embeddings_far.to_local_vec().mat_vec(&state.x, &mut state.logits);
        } else {
            let w = self.wcls.as_ref().unwrap();
            w.to_local_vec().mat_vec(&state.x, &mut state.logits);
        }
    }
}

// f32 Implementation of Llama2 layer
impl LlamaLayer<Vec<Ty>> for LayerWeights<Vec<Ty>>
{
    fn rms_and_qkv(&self, config: &Config, state: &mut ExecutionState<Vec<Ty>>) {
        self.rms_attn.to_local_vec().rms_norm(&state.x, &mut state.xb);
        self.wq.to_local_vec().mat_vec(&state.xb, &mut state.q);
        self.wk.to_local_vec().mat_vec(&state.xb, &mut state.k);
        self.wv.to_local_vec().mat_vec(&state.xb, &mut state.v);
    }

    fn rope(&self, pos: usize, cfg: &Config, state: &mut ExecutionState<Vec<Ty>>, rope_imag: &Vec<Ty>, rope_real: &Vec<Ty>) {
        let head_size = cfg.dim / cfg.n_heads;

        let q_heads = state.q.chunks_exact_mut(head_size);
        let k_heads  = state.k.chunks_exact_mut(head_size);

        for (q, k) in q_heads.zip(k_heads) {
            let mut re = rope_real[pos * head_size / 2..].iter().take(head_size / 2);
            let mut im = rope_imag[pos * head_size / 2..].iter().take(head_size / 2);

            for (qq, kk) in q.chunks_exact_mut(2).zip(k.chunks_exact_mut(2)) {
                let (q0, q1) = (qq[0], qq[1]);
                let (k0, k1) = (kk[0], kk[1]);
                let fcr = re.next().unwrap();
                let fci = im.next().unwrap();
                qq[0] = q0 * fcr - q1 * fci;
                qq[1] = q0 * fci + q1 * fcr;
                kk[0] = k0 * fcr - k1 * fci;
                kk[1] = k0 * fci + k1 * fcr;
            }
        }
    }

    fn cache_kv(&mut self, pos: usize, cfg: &Config, state: &ExecutionState<Vec<Ty>>) {
        let dst_k = &mut self.k_cache[pos * cfg.dim..(pos + 1) * cfg.dim];
        let dst_v = &mut self.v_cache[pos * cfg.dim..(pos + 1) * cfg.dim];
        dst_k.copy_from_slice(&state.k);
        dst_v.copy_from_slice(&state.v);
    }

    fn attention(&self, pos: usize, cfg: &Config, state: &ExecutionState<Vec<Ty>>) {
        let head_size = cfg.dim / cfg.n_heads;
        let k_cache = self.k_cache.as_slice();
        let v_cache = self.v_cache.as_slice();

        (0..cfg.n_heads).for_each(|h: usize| {
            let q = unsafe { _unchecked_slice(&state.q, h * head_size, head_size) };
            // head attention weights on len (seq_len, )
            let att_weights = unsafe { _unchecked_mut_slice(&state.att, h * cfg.seq_len, cfg.seq_len) };
            let xb = unsafe { _unchecked_mut_slice(&state.xb, h * head_size, head_size) };
            // head K cache of (seq_len,head_size)
            let mut head_k_cache = k_cache.chunks_exact(head_size).skip(h).step_by(cfg.n_heads);

            // do <Q,K> for head
            for t in 0..=pos {
                let k = head_k_cache.next().unwrap(); // head_size
                let score = k
                    .iter()
                    .zip(q.iter())
                    .fold(0 as Ty, |acc, (_k, _q)| acc + _k * _q);
                let score = score / (head_size as Ty).sqrt();
                unsafe {
                    *att_weights.get_unchecked_mut(t) = score;
                }
            }

            // head V cache of (seq_len, head_size)
            let head_v_cache = v_cache.chunks_exact(head_size).skip(h).step_by(cfg.n_heads);
            inplace_softmax(&mut att_weights[..=pos]);
            // reset buffer head out buffer
            xb.iter_mut().for_each(|v| *v = 0 as Ty);
            // accumulate cached values to current buffer
            // according to attention prob. (normalized weights)
            for (vals, p_attn) in head_v_cache.zip(att_weights.iter()).take(pos + 1) {
                vals.iter()
                    .zip(xb.iter_mut())
                    .for_each(|(v, dst)| *dst += v * p_attn)
            }
        });
    }

    fn merge_heads_to_resid_stream(&self, state: &mut ExecutionState<Vec<Ty>>) {
        // merge heads
        // at this point result of all heads is in x[1],
        // Linearly merge all heads into a new buffer x[2]
        self.wo.to_local_vec().mat_vec(&state.xb, &mut state.xb2);

        // add attention result to residual stream
        state
            .x
            .iter_mut()
            .zip(state.xb2.iter())
            .for_each(|(x, xb)| *x += *xb);
    }

    fn ffn(&self, state: &mut ExecutionState<Vec<Ty>>) {
        // normalize redisual stream before FFN
        self.rms_ffn.to_local_vec().rms_norm(&state.x, &mut state.xb);

        // FFN:
        // z = SiLU(W1 \dot x) * (W3 \dot x)
        // out = (W2 \dot z)
        self.w1.to_local_vec().mat_vec(&state.xb, &mut state.h1);
        self.w3.to_local_vec().mat_vec(&state.xb, &mut state.h2);

        // silu hidden
        for h1 in state.h1.iter_mut() {
            // 1 / 1 + exp(-hv)
            let _scaler = (1 as Ty) / ((1 as Ty) + (-*h1).exp());
            *h1 = *h1 * _scaler;
        }

        // combine hidden state with multiplication
        for (h1, &h2) in state.h1.iter_mut().zip(state.h2.iter()) {
            *h1 *= h2;
        }
        self.w2.to_local_vec().mat_vec(&state.h1, &mut state.xb);

        // add FFN result to residual stream
        state
            .x
            .iter_mut()
            .zip(state.xb.iter())
            .for_each(|(x, z)| *x += *z);
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

#[inline]
fn _norm_const(vec: &[Ty]) -> Ty {
    let dim = vec.len() as Ty;
    let ssq = vec.iter().fold(0f32, |init, &v| init + v * v) / dim;
    (1 as Ty) / (ssq + 1e-5).sqrt()
}

fn inplace_softmax(x: &mut [Ty]) {
    let max_val = x.iter().fold(Ty::NAN, |acc, &v| v.max(acc));
    let mut denom = 0 as Ty;
    for v in x.iter_mut() {
        *v = (*v - max_val).exp();
        denom += *v;
    }

    x.iter_mut().for_each(|v| *v /= denom);
}

fn cdf_sample(probs: &[Ty]) -> usize {
    let mut small_rng = SmallRng::from_entropy();

    let r = small_rng.gen::<Ty>();
    let mut cdf = 0 as Ty;
    for (idx, p) in probs.iter().enumerate() {
        cdf += *p;
        if r < cdf {
            return idx;
        }
    }
    probs.len() - 1
}

fn matmul(out: &mut [Ty], x: &[Ty], w: &[Ty]) {
    let stride = x.len();
    for (row, out_elem) in w.chunks_exact(stride).zip(out.iter_mut()) {
        *out_elem = row
            .iter()
            .zip(x.iter())
            .fold(0 as Ty, |acc, (&_w, &_x)| acc + _w * _x);
    }
}

/// We can safely borrow disjoin parts of slices, but its really hard for the borrow checker to know that this is safe
unsafe fn _unchecked_mut_slice(s: &[Ty], offset: usize, size: usize) -> &mut [Ty] {
    let ptr: *mut f32 = s.as_ptr() as *mut Ty;
    let st = ptr.add(offset);
    std::slice::from_raw_parts_mut(st, size)
}

/// We can safely borrow disjoin parts of slices, but its really hard for the borrow checker to know that this is safe
unsafe fn _unchecked_slice<Q>(s: &[Q], offset: usize, size: usize) -> &[Q] {
    let ptr = s.as_ptr();
    let st = ptr.add(offset);
    std::slice::from_raw_parts(st, size)
}

pub fn run_llm_inference_demo(
    metrics: Registry,
    run_id: String,
    token: &str,
    storage_endpoints: Vec<String>,
    manager_endpoint: Option<String>,
    time_limit: u64,
    optimize: bool,
    memory_limit: Option<u64>,
    replacement_policy: Option<Box<dyn ReplacementPolicy>>
) -> f32 {
    info!("running llm inference demo");

    let slo = 5.45;

    if optimize {
        info!("running in optimization mode");

        let mut memory_threshold = 26000 * 1024 * 1024;
        loop {
            info!("trying {}MB as local memory treshold", memory_threshold / (1024 * 1024));

            let total_tokens = run_inference(metrics.clone(), run_id.clone(), token, storage_endpoints.clone(), manager_endpoint.clone(), time_limit, memory_threshold, None);
            if (15.0 * 60.0 / total_tokens as f32) < slo {
                break;
            }

            memory_threshold -= 100 * 1024 * 1024;
        }

        info!("lowest local memory threshold which maintains SLO is {}MB", memory_threshold / (1024 * 1024));
        0.0
    } else {
        run_inference(metrics, run_id.clone(), token, storage_endpoints, manager_endpoint, time_limit, memory_limit.unwrap_or(25600 * 1024 * 1024), replacement_policy) as f32
    }
}

fn run_inference(
    metrics: Registry,
    run_id: String,
    token: &str,
    storage_endpoints: Vec<String>,
    manager_endpoint: Option<String>,
    time_limit: u64,
    local_max_memory: u64,
    replacement_policy: Option<Box<dyn ReplacementPolicy>>
) -> u32 {
    let manager_client = manager_endpoint.map(|endpoint| {
        let mut client = ManagerClient::new(&endpoint);
        client.auth(token);
        client
    });

    let backend: Box<dyn FarMemoryBackend> = if !storage_endpoints.is_empty() {
        if storage_endpoints.len() == 1 {
            info!("running in single backend node mode");
            Box::new(NetworkNodeBackend::new(&storage_endpoints[0], token, run_id))
        } else if storage_endpoints.len() == 5 {
            info!("running in erasure coded mode");

            let nodes: Vec<_> = storage_endpoints.iter()
                .map(|v| Box::new(NetworkNodeBackend::new(&v, token, run_id.clone())) as Box<dyn FarMemoryBackend>)
                .collect();

            Box::new(ErasureCodingBackend::new(nodes))
        } else {
            let nodes: Vec<_> = storage_endpoints.iter()
                .map(|v| Box::new(NetworkNodeBackend::new(&v, token, run_id.clone())) as Box<dyn FarMemoryBackend>)
                .collect();

            info!("running in replication mode with {} nodes", nodes.len());

            Box::new(ReplicationBackend::new(nodes))
        }
    } else {
        warn!("no storage endpoint provided, falling back to disk backend");
        Box::new(LocalDiskBackend::new())
    };

    let backend = Box::new(InstrumentedBackend::new(metrics.clone(), backend));

    let mut client = FarMemoryClient::new(backend, local_max_memory);
    if let Some(manager) = manager_client {
        let fallback = PreferRemoteSpansReplacementPolicy::new(Box::new(MostRecentlyUsedReplacementPolicy::new()));

        if replacement_policy.is_none() {
            client.use_replacement_policy(Box::new(RemoteReplayReplacementPolicy::new(manager.clone(), Box::new(fallback))));
        }
        client.use_manager(manager);
    }
    if let Some(policy) = replacement_policy {
        client.use_replacement_policy(policy);
    }
    client.track_metrics(metrics.clone());
    client.start_swap_out_thread();

    let llama = true;

    let model_path = if llama {
        "./data/llama2_7b_chat.bin"
    } else {
        "./data/stories15M.bin"
    };

    let temperature = 0 as Ty;
    let tokenizer_path = "./data/tokenizer.bin";

    let config = Config::from_file(&model_path);
    let seq_len = config.seq_len;

    let encoded_prompt = [1, 390, 504, 338, 278, 1900, 8720, 4086, 1363]; // Rust is the best programming language because

    let vocab = Vocab::from_file(client.clone(), config.vocab_size, tokenizer_path);
    let mut weights = LlamaWeights::load_weights(client.clone(), &config, &model_path);

    let mut state = ExecutionState::<Vec<Ty>>::init(&config);
    let mut probs = vec![0 as Ty; config.vocab_size];

    let mut pos = 0;
    let mut token = 1;

    for prompt_token in encoded_prompt {
        weights.step(prompt_token, pos, &config, &mut state);
        pos += 1;
        token = prompt_token;

        if pos > 1 {
            print!("{}", vocab.get_token(token));
            io::stdout().flush().unwrap();
        }
    }

    let started_at = Instant::now();
    let mut time_per_token = CKMS::<f32>::new(0.001);
    let mut total_tokens_generated = 0;
    let mut memory_usage_megabytes: CKMS<f64> = CKMS::<f64>::new(0.001);
    let mut memory_usage_far_local_memory: CKMS<f64> = CKMS::<f64>::new(0.001);
    let mut memory_usage_far_remote_memory: CKMS<f64> = CKMS::<f64>::new(0.001);

    let metric_total_generation_time = register_gauge_with_registry!(
        "demo_generation_time",
        "total time in seconds spent generating tokens",
        metrics
    ).unwrap();
    let metric_total_tokens_generated = register_int_counter_with_registry!(
        "demo_tokens_generated",
        "total tokens generated",
        metrics
    ).unwrap();

    while pos < seq_len && (Instant::now() - started_at).as_secs() < time_limit {
        let token_started_at = Instant::now();

        weights.step(token, pos, &config, &mut state);

        let next = if temperature == 0 as Ty {
            state
                .logits
                .iter()
                .enumerate()
                .max_by(|(_, a), (_, b)| a.total_cmp(b))
                .map(|(index, _)| index)
                .unwrap()
        } else {
            state
                .logits
                .iter()
                .zip(probs.iter_mut())
                .for_each(|(logit, p)| *p = logit / temperature);
            inplace_softmax(&mut probs);
            cdf_sample(&probs)
        };

        let token_time = (Instant::now() - token_started_at).as_secs_f32();
        time_per_token.insert(token_time);
        memory_usage_megabytes.insert((current_memory_usage() / (1024 * 1024)) as f64);
        memory_usage_far_local_memory.insert((client.total_local_memory() / (1024 * 1024)) as f64);
        memory_usage_far_remote_memory.insert((client.total_remote_memory() / (1024 * 1024)) as f64);

        print!("{}", vocab.get_token(next));
        io::stdout().flush().unwrap();
        pos += 1;
        token = next;
        total_tokens_generated += 1;

        metric_total_tokens_generated.inc();
        metric_total_generation_time.set((Instant::now() - started_at).as_secs_f64());
    }

    println!("");

    client.stop();

    metrics.unregister(Box::new(metric_total_generation_time.clone())).unwrap();
    metrics.unregister(Box::new(metric_total_tokens_generated.clone())).unwrap();

    println!(
        "done, total tokens generated: {}, total time: {} seconds, time per token avg: {} seconds, p95: {} seconds",
        total_tokens_generated,
        (Instant::now() - started_at).as_secs_f32(),
        time_per_token.query(0.5).unwrap().1,
        time_per_token.query(0.9).unwrap().1
    );

    println!(
        "average memory usage: {} MB (local), {} MB (far local), {} MB (far remote)",
        memory_usage_megabytes.query(0.5).unwrap().1,
        memory_usage_far_local_memory.query(0.5).unwrap().1,
        memory_usage_far_remote_memory.query(0.5).unwrap().1
    );

    total_tokens_generated
}
