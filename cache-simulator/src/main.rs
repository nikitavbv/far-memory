use std::{collections::HashMap, marker::PhantomData, sync::{Arc, RwLock}};

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
    fn next_item(&self) -> Request<T>;
}

trait WorkloadHandler<T> {
    fn handle(&self, request: &Request<T>) -> Response;
}

trait TimeHandler {
    fn tick(&self, delta: u64);
}

struct Cache<R, H: WorkloadHandler<R>> {
    request_type: PhantomData<R>,
    cache: Arc<RwLock<HashMap<RequestId, u64>>>,
    handler: H,
    ttl: u64,
    size_limit: usize,
}

impl<K, E: WorkloadHandler<K>> Cache<K, E> {
    pub fn new(handler: E, ttl: u64, size_limit: usize) -> Self {
        Self {
            request_type: PhantomData,
            cache: Arc::new(RwLock::new(HashMap::new())),
            handler,
            ttl,
            size_limit,
        }
    }
}

impl<R, H: WorkloadHandler<R>> WorkloadHandler<R> for Cache<R, H> {
    fn handle(&self, request: &Request<R>) -> Response {
        if self.cache.read().unwrap().contains_key(&request.id) {
            *self.cache.write().unwrap().get_mut(&request.id).unwrap() = self.ttl;
            return Response {
                time_units: 0.005,
                compute_units: 0.001,
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
            time_units: result.time_units + 0.1,
            compute_units: result.time_units + 0.01,
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
}

impl<R, H: WorkloadHandler<R>> ComputeNode<R, H> {
    pub fn new(handler: H, initial_compute_units: f64) -> Self {
        Self {
            request_type: PhantomData,
            handler,
            compute_units_available: RwLock::new(initial_compute_units),
        }
    }

    pub fn handle_request(&self, request: &Request<R>) -> Option<Response> {
        if *self.compute_units_available.read().unwrap() < 0.0 {
            None
        } else {
            let response = self.handler.handle(request);
            *self.compute_units_available.write().unwrap() -= response.compute_units;
            Some(response)
        }
    }
}

struct ComputeCluster<R, H: WorkloadHandler<R> + Clone> {
    request_type: PhantomData<R>,
    handler: H,
    nodes: Vec<ComputeCluster<R, H>>,
}

fn main() {
    println!("Hello, world!");
}

struct SummarizeYoutubeVideoRequest {
    video_duration: f64,
}

struct YoutubeVideoSummarizer {
}

impl WorkloadHandler<SummarizeYoutubeVideoRequest> for YoutubeVideoSummarizer {
    fn handle(&self, request: &Request<SummarizeYoutubeVideoRequest>) -> Response {
        Response {
            time_units: 0.1 + 0.2 * request.body.video_duration,
            compute_units: 0.05 + 0.15 * request.body.video_duration,
        }
    }
}