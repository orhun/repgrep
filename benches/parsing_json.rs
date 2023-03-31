use std::{
    fs::File,
    io::{BufRead, BufReader, Read},
    sync::Arc,
    thread,
    time::Duration,
};

use criterion::{criterion_group, criterion_main, Criterion};
use crossbeam_queue::ArrayQueue;
use serde_json::Deserializer;

use librepgrep::rg::de::RgMessage;

const RG_JSON_PATH: &str = "benches/rg.json";

fn bufreader_lines() -> Vec<RgMessage> {
    let file = File::open(RG_JSON_PATH).unwrap();
    let reader = BufReader::new(file);
    let mut items = vec![];

    for line in reader.lines() {
        items.push(serde_json::from_str::<RgMessage>(&line.unwrap()).unwrap());
    }

    items
}

fn bufreader_stream() -> Vec<RgMessage> {
    let file = File::open(RG_JSON_PATH).unwrap();
    let reader = BufReader::new(file);
    let stream = Deserializer::from_reader(reader);
    stream
        .into_iter()
        .map(|x| x.unwrap())
        .collect::<Vec<RgMessage>>()
}

// fastest, but comes at a 2x memory cost
fn read_all_and_par_iter() -> Vec<RgMessage> {
    use rayon::prelude::*;

    let mut file = File::open(RG_JSON_PATH).unwrap();
    let mut s = String::new();
    file.read_to_string(&mut s).unwrap();
    s.lines()
        .par_bridge()
        .map(|x| serde_json::from_str::<RgMessage>(&x).unwrap())
        .collect::<Vec<RgMessage>>()
}

fn crossbeam_queue() -> Vec<RgMessage> {
    let file = File::open(RG_JSON_PATH).unwrap();
    let reader = BufReader::new(file);

    let q = Arc::new(ArrayQueue::new(128));
    let thread_q = q.clone();
    let t = thread::spawn(move || {
        for line in reader.lines() {
            let mut line = line.unwrap();
            loop {
                match thread_q.push(line) {
                    Ok(_) => break,
                    Err(value) => line = value,
                }
            }
        }
    });

    let mut items = vec![];
    while !t.is_finished() {
        while let Some(line) = q.pop() {
            items.push(serde_json::from_str(&line).unwrap());
        }
    }

    items
}

// TODO: is there a way to read parts of the file in parallel?
//      get file size, start reading until first \n at 0% 25% 50% and 75%, use that to par read and parse

fn criterion_benchmark(c: &mut Criterion) {
    let mut g = c.benchmark_group("parsing json");
    g.measurement_time(Duration::from_secs(20));

    g.bench_function("BufReader::lines", |b| b.iter(|| bufreader_lines()));
    g.bench_function("StreamDeserializer", |b| b.iter(|| bufreader_stream()));
    g.bench_function("Read & ParallelIter", |b| {
        b.iter(|| read_all_and_par_iter())
    });
    g.bench_function("Read & Parse at the same time", |b| {
        b.iter(|| crossbeam_queue())
    });

    g.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
