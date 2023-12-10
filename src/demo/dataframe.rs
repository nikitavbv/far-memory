use {
    std::{fs::File, time::Instant, hint::black_box},
    tracing::{info, warn},
    prometheus::Registry,
    serde::{Serialize, Deserialize},
    chrono::NaiveDate,
    rand::Rng,
    rand_distr::Zipf,
    crate::{
        client::{
            FarMemoryClient,
            FarMemoryBackend,
            NetworkNodeBackend,
            ErasureCodingBackend,
            ReplicationBackend,
            LocalDiskBackend,
            InstrumentedBackend,
            PreferRemoteSpansReplacementPolicy,
            MostRecentlyUsedReplacementPolicy,
            RemoteReplayReplacementPolicy,
            FarMemorySerializedObjectSet,
        },
        manager::ManagerClient,
    },
};

#[derive(Serialize, Deserialize, Debug, Default)]
struct FlightData {
    flight_date: NaiveDate,
    airline: String,
    origin: String,
    destination: String,
    cancelled: bool,
    diverted: bool,
    crs_dep_time: u16,
    dep_time: Option<f32>,
    dep_delay_minutes: Option<f32>,
    dep_delay: Option<f32>,
    arr_time: Option<f32>,
    arr_delay_minutes: Option<f32>,
    air_time: Option<f32>,
    crs_elapsed_time: Option<f32>,
    actual_elapsed_time: Option<f32>,
    distance: f32,
    year: u32,
    quarter: u8,
    month: u8,
    day_of_month: u8,
    day_of_week: u8,
    marketing_airline_network: String,
    operated_or_branded_code_share_partners: String,
    dot_id_marketing_airline: u32,
    iata_code_marketing_airline: String,
    flight_number_marketing_airline: u32,
    operating_airline: String,
    dot_id_operating_airline: u32,
    iata_code_operating_airline: String,
    tail_number: String,
    flight_number_operating_airline: u32,
    origin_airport_id: u32,
    origin_airport_seq_id: u32,
    origin_city_market_id: u32,
    origin_city_name: String,
    origin_city_state: String,
    origin_state_fips: u32,
    origin_state_name: String,
    origin_wac: u32,
    dest_airport_id: u32,
    dest_airport_seq_id: u32,
    dest_city_market_id: u32,
    dest_city_name: String,
    dest_state: String,
    dest_state_fips: u32,
    dest_state_name: String,
    dest_wac: u32,
    dest_del_15: Option<f32>,
    departure_delay_groups: Option<f32>,
    dep_time_blk: String,
    taxi_out: Option<f32>,
    wheels_off: Option<f32>,
    wheels_on: Option<f32>,
    taxi_in: Option<f32>,
    crs_arr_time: u32,
    arr_delay: Option<f32>,
    arr_del_15: Option<f32>,
    arrival_delay_groups: Option<f32>,
    arr_time_blk: String,
    distance_group: u32,
    div_airport_landings: Option<f32>,
}

#[derive(Debug)]
struct FlightsQuery {
    after_date: Option<NaiveDate>,
    origin_airport_id: Option<u32>,
    airline_code: Option<u32>, // based on dot_id_operating_airline
}

struct DemoDataFramePipeline {
    dataframe: FarMemorySerializedObjectSet<FlightData>,
}

impl DemoDataFramePipeline {
    pub fn new(dataframe: FarMemorySerializedObjectSet<FlightData>) -> Self {
        Self {
            dataframe,
        }
    }

    pub fn pick_random(&self, zipf_s: f32) -> FlightData {
        let index = rand::thread_rng().sample(Zipf::new(self.dataframe.len() as u64, zipf_s).unwrap()).round() as u64 - 1; // -1 because zipf returns [1; n]
        self.dataframe.get(index as usize)
    }

    /* get average delay based on arr_delay */
    pub fn get_average_delay_with_criteria(&self, query: FlightsQuery) -> Option<f32> {
        let (total_delay, total_objects) = self.dataframe
            .iter()
            .filter(|v| query.after_date.is_none() || query.after_date.unwrap() > v.flight_date)
            .filter(|v| query.airline_code.is_none() || query.airline_code.unwrap() == v.dot_id_operating_airline)
            .filter(|v| query.origin_airport_id.is_none() || query.origin_airport_id.unwrap() == v.origin_airport_id)
            .filter(|v| v.arr_delay.is_some())
            .fold((0.0, 0), |acc, v| (acc.0 + v.arr_delay.unwrap(), acc.1 + 1));

        if total_objects == 0 {
            None
        } else {
            Some(total_delay / total_objects as f32)
        }
    }
}

/**
This demo is designed to rely on streaming to demonstrate optimizations that are available in
such case.

using this dataset: https://www.kaggle.com/datasets/robikscube/flight-delay-dataset-20182022/
*/
pub fn run_dataframe_demo(metrics: Registry, run_id: String, token: &str, storage_endpoints: Vec<String>, manager_endpoint: Option<String>, local_max_memory: Option<u64>) {
    info!("running dataframe demo");

    // far memory client init
    let local_max_memory = local_max_memory.unwrap_or(20 * 1024 * 1024 * 1024);

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

        client.use_replacement_policy(Box::new(RemoteReplayReplacementPolicy::new(manager.clone(), Box::new(fallback))));
        client.use_manager(manager);
    }
    client.track_metrics(metrics.clone());
    client.start_swap_out_thread();

    // demo app
    let mut dataframe: FarMemorySerializedObjectSet<FlightData> = FarMemorySerializedObjectSet::new(client.clone());
    let dataframe_size_limit = 1_000_000; // 20M for 12GB memory.

    'loading: loop {
        for year in 2018..2023 {
            let file_name = format!("./data/flights/Combined_Flights_{}.csv", year);
            let mut reader = csv::Reader::from_reader(File::open(file_name).unwrap());
            for row in reader.records() {
                let row = row.unwrap();
                dataframe.push(FlightData {
                    flight_date: NaiveDate::parse_from_str(&row[0], "%Y-%m-%d").unwrap(),
                    airline: row[1].to_owned(),
                    origin: row[2].to_owned(),
                    destination: row[3].to_owned(),
                    cancelled: parse_bool(&row[4]),
                    diverted: parse_bool(&row[5]),
                    crs_dep_time: row[6].parse().unwrap(),
                    dep_time: parse_option_f32(&row[7]),
                    dep_delay_minutes: parse_option_f32(&row[8]),
                    dep_delay: parse_option_f32(&row[9]),
                    arr_time: parse_option_f32(&row[10]),
                    arr_delay_minutes: parse_option_f32(&row[11]),
                    air_time: parse_option_f32(&row[12]),
                    crs_elapsed_time: parse_option_f32(&row[13]),
                    actual_elapsed_time: parse_option_f32(&row[14]),
                    distance: row[15].parse().unwrap(),
                    year: row[16].parse().unwrap(),
                    quarter: row[17].parse().unwrap(),
                    month: row[18].parse().unwrap(),
                    day_of_month: row[19].parse().unwrap(),
                    day_of_week: row[20].parse().unwrap(),
                    marketing_airline_network: row[21].parse().unwrap(),
                    operated_or_branded_code_share_partners: row[22].parse().unwrap(),
                    dot_id_marketing_airline: row[23].parse().unwrap(),
                    iata_code_marketing_airline: row[24].parse().unwrap(),
                    flight_number_marketing_airline: row[25].parse().unwrap(),
                    operating_airline: row[26].parse().unwrap(),
                    dot_id_operating_airline: row[27].parse().unwrap(),
                    iata_code_operating_airline: row[28].parse().unwrap(),
                    tail_number: row[29].parse().unwrap(),
                    flight_number_operating_airline: row[30].parse().unwrap(),
                    origin_airport_id: row[31].parse().unwrap(),
                    origin_airport_seq_id: row[32].parse().unwrap(),
                    origin_city_market_id: row[33].parse().unwrap(),
                    origin_city_name: row[34].parse().unwrap(),
                    origin_city_state: row[35].parse().unwrap(),
                    origin_state_fips: row[36].parse().unwrap(),
                    origin_state_name: row[37].parse().unwrap(),
                    origin_wac: row[38].parse().unwrap(),
                    dest_airport_id: row[39].parse().unwrap(),
                    dest_airport_seq_id: row[40].parse().unwrap(),
                    dest_city_market_id: row[41].parse().unwrap(),
                    dest_city_name: row[42].parse().unwrap(),
                    dest_state: row[43].parse().unwrap(),
                    dest_state_fips: row[44].parse().unwrap(),
                    dest_state_name: row[45].parse().unwrap(),
                    dest_wac: row[46].parse().unwrap(),
                    dest_del_15: parse_option_f32(&row[47]),
                    departure_delay_groups: parse_option_f32(&row[48]),
                    dep_time_blk: row[49].parse().unwrap(),
                    taxi_out: parse_option_f32(&row[50]),
                    wheels_off: parse_option_f32(&row[51]),
                    wheels_on: parse_option_f32(&row[52]),
                    taxi_in: parse_option_f32(&row[53]),
                    crs_arr_time: row[54].parse().unwrap(),
                    arr_delay: parse_option_f32(&row[55]),
                    arr_del_15: parse_option_f32(&row[56]),
                    arrival_delay_groups: parse_option_f32(&row[57]),
                    arr_time_blk: row[58].parse().unwrap(),
                    distance_group: row[59].parse().unwrap(),
                    div_airport_landings: parse_option_f32(&row[60]),
                });

                if dataframe.len() >= dataframe_size_limit {
                    break 'loading;
                }
            }
        }
    }
    println!("finished loading data");

    // run queries
    let dataframe = DemoDataFramePipeline::new(dataframe);
    let mut total_queries = 0;
    let started_at = Instant::now();
    let zipf_s = 0.8;

    let mut checkpoint = Instant::now();

    loop {
        let now = Instant::now();
        let time_since_start = (now - started_at).as_secs();
        if time_since_start > 15 * 60 {
            break;
        }

        let flight = black_box(dataframe.pick_random(zipf_s));
        let query = random_query_for_similar_flights(flight);

        let _avg = black_box(dataframe.get_average_delay_with_criteria(black_box(query)));

        total_queries += 1;

        if (now - checkpoint).as_secs() > 60 {
            checkpoint = Instant::now();
            println!("operations per second: {}", total_queries / time_since_start);
        }
    }

    println!("result: operations per second: {}", total_queries / (Instant::now() - started_at).as_secs());
}

fn random_query_for_similar_flights(flight: FlightData) -> FlightsQuery {
    let mut query = FlightsQuery {
        after_date: None,
        origin_airport_id: None,
        airline_code: None,
    };

    if rand::thread_rng().gen_bool(0.7) {
        query.after_date = Some(flight.flight_date);
    }

    if rand::thread_rng().gen_bool(0.7) {
        query.origin_airport_id = Some(flight.origin_airport_id);
    }

    if rand::thread_rng().gen_bool(0.7) {
        query.airline_code = Some(flight.dot_id_operating_airline);
    }

    query
}

fn parse_bool(s: &str) -> bool {
    let s = s.to_lowercase();
    match s.parse() {
        Ok(v) => v,
        Err(_) => panic!("failed to parse \"{}\" as bool", s),
    }
}

fn parse_option_f32(s: &str) -> Option<f32> {
    if s.is_empty() {
        None
    } else {
        Some(s.parse().unwrap())
    }
}
