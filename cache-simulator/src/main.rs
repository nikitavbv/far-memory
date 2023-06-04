use {
    std::{collections::{HashMap, VecDeque}, marker::PhantomData, sync::{Arc, RwLock}, fs},
    rand::{
        prelude::*,
        distributions::WeightedIndex,
    },
    rand_chacha::ChaCha20Rng,
};

type RequestId = u64;

struct Request<T> {
    id: RequestId,
    body: T,
}

struct Response {
    time_units: f64,
    compute_units: f64,
}

trait WorkloadGenerator<T> {
    fn next_item(&mut self) -> Request<T>;
}

trait WorkloadHandler<T> {
    fn handle(&self, request: &Request<T>) -> Response;
}

trait TimeHandler {
    fn tick(&self, delta: u64);
}

#[derive(Clone)]
struct Cache<R, H: WorkloadHandler<R>> {
    request_type: PhantomData<R>,
    cache: Arc<RwLock<HashMap<RequestId, u64>>>,
    handler: H,
    ttl: u64,
    size_limit: usize,

    read_latency: f64,
    read_compute: f64,
    write_latency: f64,
    write_compute: f64,
}

impl<K, E: WorkloadHandler<K>> Cache<K, E> {
    pub fn new(handler: E, ttl: u64, size_limit: usize, read_latency: f64, read_compute: f64, write_latency: f64, write_compute: f64) -> Self {
        Self {
            request_type: PhantomData,
            cache: Arc::new(RwLock::new(HashMap::new())),
            handler,
            ttl,
            size_limit,
            read_latency, // 0.005
            read_compute, // 0.001
            write_latency, // 0.1
            write_compute, // 0.01
        }
    }
}

impl<R, H: WorkloadHandler<R>> WorkloadHandler<R> for Cache<R, H> {
    fn handle(&self, request: &Request<R>) -> Response {
        if self.cache.read().unwrap().contains_key(&request.id) {
            *self.cache.write().unwrap().get_mut(&request.id).unwrap() = self.ttl;
            return Response {
                time_units: self.read_latency,
                compute_units: self.read_compute,
            };
        };

        let result = self.handler.handle(request);
        self.cache.write().unwrap().insert(request.id, self.ttl);

        let mut cache = self.cache.write().unwrap();
        while cache.len() > self.size_limit {
            let mut lowest_ttl: Option<(RequestId, u64)> = None;
            for element in cache.iter() {
                if lowest_ttl.is_none() || element.1 < &lowest_ttl.unwrap().1 {
                    lowest_ttl = Some((*element.0, *element.1));
                }
            }
            cache.remove(&lowest_ttl.unwrap().0);
        }

        Response {
            time_units: result.time_units + self.write_latency,
            compute_units: result.compute_units + self.write_compute,
        }
    }
}

impl<R, H: WorkloadHandler<R>> TimeHandler for Cache<R, H> {
    fn tick(&self, delta: u64) {
        let mut cache = self.cache.write().unwrap();
        for (request_id, ttl) in cache.clone().iter() {
            if &delta > ttl {
                cache.remove(request_id);
            } else {
                cache.insert(*request_id, ttl - delta);
            }
        }
    }
}

struct ComputeNode<R, H: WorkloadHandler<R>> {
    request_type: PhantomData<R>,
    handler: H,
    compute_units_available: RwLock<f64>,
    cores: u64,
}

impl<R, H: WorkloadHandler<R>> ComputeNode<R, H> {
    pub fn new(handler: H, initial_compute_units: f64) -> Self {
        Self {
            request_type: PhantomData,
            handler,
            compute_units_available: RwLock::new(initial_compute_units),
            cores: 10,
        }
    }

    pub fn handle_request(&self, request: &Request<R>) -> Option<Response> {
        let response = self.handler.handle(request);

        if *self.compute_units_available.read().unwrap() < response.compute_units {
            None
        } else {
            *self.compute_units_available.write().unwrap() -= response.compute_units;
            Some(response)
        }
    }
}

impl <R, H: WorkloadHandler<R> + TimeHandler> TimeHandler for ComputeNode<R, H> {
    fn tick(&self, delta: u64) {
        self.handler.tick(delta);
        *self.compute_units_available.write().unwrap() += (delta as f64) * self.cores as f64;
        let max = 10.0 * self.cores as f64;
        if *self.compute_units_available.read().unwrap() > max {
            *self.compute_units_available.write().unwrap() = max;
        }
    }
}

struct ComputeCluster<R, H: WorkloadHandler<R> + Clone> {
    request_type: PhantomData<R>,
    nodes: Vec<ComputeNode<R, H>>,
}

impl <R, H: WorkloadHandler<R> + Clone> ComputeCluster<R, H> {
    pub fn new(number_of_nodes: usize, handler: H, initial_compute_units: f64) -> Self {
        Self {
            request_type: PhantomData,
            nodes: (0..number_of_nodes).into_iter().map(|_| ComputeNode::new(handler.clone(), initial_compute_units)).collect(),
        }
    }

    pub fn of(nodes: Vec<ComputeNode<R, H>>) -> Self {
        Self {
            request_type: PhantomData,
            nodes,
        }
    }

    pub fn handle_request(&self, request: &Request<R>) -> Option<Response> {
        for node in &self.nodes {
            if let Some(res) = node.handle_request(request) {
                return Some(res);
            }
        }

        None
    }
}

impl <R, H: WorkloadHandler<R> + TimeHandler + Clone> TimeHandler for ComputeCluster<R, H> {
    fn tick(&self, delta: u64) {
        for node in &self.nodes {
            node.tick(delta);
        }
    }
}

fn main() {
    println!("cache usage simulator");

    let mut rng: ChaCha20Rng = ChaCha20Rng::seed_from_u64(42);

    let mut workload_generator = YoutubeVideoWorkloadGenerator::new(&mut rng);
    println!("workload generator ready");

    let summarizer = YoutubeVideoSummarizer;
    let cache = Cache::new(summarizer, 200, 2000, 0.005, 0.0005, 0.01, 0.001);
    let summarizer = cache;

    let local_ttl = 200;
    let local_cache_size = 1000;
    let local_read_latency = 0.0005;
    let local_read_compute = local_read_latency;
    let local_write_latency = 0.001;
    let local_write_compute = local_write_latency;

    let cluster = ComputeCluster::of(vec![
        /*ComputeNode::new(summarizer.clone(), 1.0),
        ComputeNode::new(summarizer.clone(), 1.0),
        ComputeNode::new(summarizer.clone(), 1.0),*/

        ComputeNode::new(Cache::new(summarizer.clone(), local_ttl, local_cache_size, local_read_latency, local_read_compute, local_write_latency, local_write_compute), 1.0),
        ComputeNode::new(Cache::new(summarizer.clone(), local_ttl, local_cache_size, local_read_latency, local_read_compute, local_write_latency, local_write_compute), 1.0),
        ComputeNode::new(Cache::new(summarizer.clone(), local_ttl, local_cache_size, local_read_latency, local_read_compute, local_write_latency, local_write_compute), 1.0),
    ]);

    let mut queue = VecDeque::new();
    let mut epoch = 0;
    let total_epochs = 1000;
    let max_epochs = 2000;

    while epoch < total_epochs || !queue.is_empty() {
        cluster.tick(1);

        if epoch > max_epochs {
            break;
        }

        if epoch < total_epochs {
            let requests_to_add = 1 + rng.gen_range(0..10);
            for _ in 0..requests_to_add {
                queue.push_back((epoch, workload_generator.next_item()));
            }
        }

        while let Some((request_epoch, request)) = queue.pop_front() {
            let response = cluster.handle_request(&request);
            if response.is_none() {
                queue.push_front((request_epoch, request));
                break;
            }
        }

        println!("epoch: {}, queue size: {}", epoch, queue.len());

        epoch += 1;
    }
}

#[derive(Clone)]
struct SummarizeYoutubeVideoRequest {
    video_duration: f64,
}


struct YoutubeVideoWorkloadGenerator {
    duration: Vec<f64>,
    rng: ChaCha20Rng,
    index: WeightedIndex<u64>,
}

impl YoutubeVideoWorkloadGenerator {
    pub fn new(rng: &mut ChaCha20Rng) -> Self {
        let mut rdr = csv::Reader::from_reader(fs::File::open("data/youtube_videos.csv").unwrap());
        let mut views: Vec<u64> = Vec::new();
        let mut duration: Vec<f64> = Vec::new();
        
        for row in rdr.records() {
            let row = row.unwrap();
            let video_views = row.get(8).unwrap();
            views.push(video_views.parse().unwrap());
            duration.push(generate_video_duration(rng));
        }

        Self {
            duration,
            rng: ChaCha20Rng::seed_from_u64(42),
            index: WeightedIndex::new(views).unwrap(),
        }
    }
}

fn generate_video_duration(rng: &mut ChaCha20Rng) -> f64 {
    let weights = vec![0.04, 0.07, 0.075, 0.135, 0.1225, 0.118, 0.95, 0.08, 0.057, 0.04];
    let index = WeightedIndex::new(&weights).unwrap();
    let index = index.sample(rng);
    (3.0 * 60.0) * (index as f64 + rng.gen::<f64>())
}

impl WorkloadGenerator<SummarizeYoutubeVideoRequest> for YoutubeVideoWorkloadGenerator {
    fn next_item(&mut self) -> Request<SummarizeYoutubeVideoRequest> {
        let id = self.index.sample(&mut self.rng);
        Request {
            id: id as u64,
            body: SummarizeYoutubeVideoRequest { video_duration: self.duration[id] },
        }
    }
}

#[derive(Clone)]
struct YoutubeVideoSummarizer;

impl WorkloadHandler<SummarizeYoutubeVideoRequest> for YoutubeVideoSummarizer {
    fn handle(&self, request: &Request<SummarizeYoutubeVideoRequest>) -> Response {
        Response {
            time_units: 0.1 + 0.2 * request.body.video_duration,
            compute_units: 0.05 + 0.015 * request.body.video_duration,
        }
    }
}

impl TimeHandler for YoutubeVideoSummarizer {
    fn tick(&self, delta: u64) {
    }
}
