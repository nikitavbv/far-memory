use {
    std::{fs::File, io::{BufReader, BufRead}, collections::HashMap},
    tracing::info,
    serde::Deserialize,
    human_bytes::human_bytes,
};

#[derive(Deserialize, Debug)]
struct TraceEvent {
    name: String,
}

pub fn run_trace_analyzer() {
    info!("running trace analyzer");

    let trace_file = File::open("./trace.json").unwrap();

    let mut size_by_event_name = HashMap::new();

    let reader = BufReader::new(trace_file);
    for line in reader.lines() {
        let line = line.unwrap();
        let line_len = line.as_bytes().len();

        if line == "[" {
            continue;
        }

        let line = if line.ends_with(",") {
            line[..line.len() - 1].to_owned()
        } else {
            line
        };

        let event: TraceEvent = match serde_json::from_str(&line) {
            Ok(v) => v,
            Err(_) => continue,
        };

        if !size_by_event_name.contains_key(&event.name) {
            size_by_event_name.insert(event.name.clone(), 0i64);
        }
        *size_by_event_name.get_mut(&event.name).unwrap() += line_len as i64;
    }

    let mut size_by_event_name: Vec<_> = size_by_event_name.into_iter().collect();
    size_by_event_name.sort_by_key(|v| -v.1);

    println!("size by event name:");
    size_by_event_name.into_iter()
        .take(10)
        .for_each(|v| println!("{}: {}", v.0, human_bytes(v.1 as f64)));
}
