use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
use reqwest;
use std::hint::black_box;
use tokio::time::{timeout, Duration};
use futures::future::join_all;

const BASE_URL: &str = "http://127.0.0.1:7878";
const SMALL_FILE: &str = "/one-file/index.html";          // ~20 bytes
const CSS_FILE: &str = "/simple-portfolio/style.css";     // ~1KB
const JS_FILE: &str = "/simple-portfolio/script.js";      // ~500 bytes
const LARGE_FILE: &str = "/large-files/100MB.bin"; 
const XLARGE_FILE: &str = "/large-files/500MB.bin";
const XXLARGE_FILE: &str = "/large-files/1GB.bin"; 

async fn single_request(path: &str) -> Result<usize, reqwest::Error> {
    let url = format!("{}{}", BASE_URL, path);
    let response = reqwest::get(&url).await?;
    let bytes = response.bytes().await?;
    Ok(bytes.len())
}

async fn concurrent_requests(path: &str, num_requests: usize) -> Result<Vec<usize>, Box<dyn std::error::Error + Send + Sync>> {
    let url = format!("{}{}", BASE_URL, path);
    
    let tasks: Vec<_> = (0..num_requests)
        .map(|_| {
            let url = url.clone();
            tokio::spawn(async move {
                let response = reqwest::get(&url).await?;
                let bytes = response.bytes().await?;
                Ok::<usize, reqwest::Error>(bytes.len())
            })
        })
        .collect();
    
    let results = join_all(tasks).await;
    let mut responses = Vec::new();
    
    for result in results {
        match result {
            Ok(Ok(response)) => responses.push(response),
            Ok(Err(e)) => return Err(Box::new(e)),
            Err(e) => return Err(Box::new(e)),
        }
    }
    
    Ok(responses)
}

fn benchmark_single_requests(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    
    let mut group = c.benchmark_group("single_requests");
    
    // Test different file sizes
    let test_files = vec![
        ("small_file", SMALL_FILE),
        ("large_file", LARGE_FILE),
        ("xlarge_file", XLARGE_FILE),
        ("xxlarge_file", XXLARGE_FILE),
        ("css_file", CSS_FILE),
        ("js_file", JS_FILE),
    ];
    
    for (name, path) in test_files {
        group.bench_function(name, |b| {
            b.iter(|| {
                let result = rt.block_on(single_request(path));
                black_box(result)
            })
        });
    }
    
    group.finish();
}

fn benchmark_concurrent_requests(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    
    let mut group = c.benchmark_group("concurrent_requests");
    group.measurement_time(Duration::from_secs(30));
    
    // Test different concurrency levels - reduced for large files
    let concurrency_levels = vec![1, 5, 10, 20];
    
    for concurrency in concurrency_levels {
        group.bench_with_input(
            BenchmarkId::new("small_file", concurrency),
            &concurrency,
            |b, &concurrency| {
                b.iter(|| {
                    let result = rt.block_on(async {
                        timeout(
                            Duration::from_secs(10),
                            concurrent_requests(SMALL_FILE, concurrency)
                        ).await
                    });
                    black_box(result)
                })
            },
        );
        
        // Only test small concurrency for large files
        if concurrency <= 5 {
            group.bench_with_input(
                BenchmarkId::new("large_file", concurrency),
                &concurrency,
                |b, &concurrency| {
                    b.iter(|| {
                        let result = rt.block_on(async {
                            timeout(
                                Duration::from_secs(30),
                                concurrent_requests(LARGE_FILE, concurrency)
                            ).await
                        });
                        black_box(result)
                    })
                },
            );
        }
    }
    
    group.finish();
}

fn benchmark_sustained_load(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    
    let mut group = c.benchmark_group("sustained_load");
    group.measurement_time(Duration::from_secs(60));
    group.sample_size(10);
    
    // Test sustained concurrent requests with small files only
    group.bench_function("sustained_10_concurrent", |b| {
        b.iter(|| {
            let result = rt.block_on(async {
                timeout(
                    Duration::from_secs(30),
                    concurrent_requests(SMALL_FILE, 10)
                ).await
            });
            black_box(result)
        })
    });
    
    group.bench_function("sustained_20_concurrent", |b| {
        b.iter(|| {
            let result = rt.block_on(async {
                timeout(
                    Duration::from_secs(30),
                    concurrent_requests(SMALL_FILE, 20)
                ).await
            });
            black_box(result)
        })
    });
    
    group.finish();
}

criterion_group!(
    benches,
    benchmark_single_requests,
    benchmark_concurrent_requests,
    benchmark_sustained_load
);
criterion_main!(benches);
