use {
    tracing::info,
    plotters::{prelude::*, coord::{Shift, types::RangedCoordf64}},
    crate::tools::evaluation::{load_evaluation_data, Experiment, EvaluationData, SpanReplacementPolicy, DemoApplicationType},
};

pub fn generate_plots() {
    info!("running plot generation");

    demo_throughput();
    throughput_distribution();
    throughput_replacement_policies();
}

fn throughput_replacement_policies() {
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
    let root_area = BitMapBackend::new("./images/plot_replacement_policies.png", (k * 55, k * 45)).into_drawing_area();
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

fn throughput_distribution() {
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
    let root_area = BitMapBackend::new("./images/plot_throughput_distribution.png", (k * 55, k * 45)).into_drawing_area();
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
}

fn demo_throughput() {
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
    let root_area = BitMapBackend::new("./images/plot_demo_throughput.png", (k * 55, k * 45)).into_drawing_area();
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
