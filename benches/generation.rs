use criterion::{criterion_group, criterion_main, Criterion};
use kyros::{self, utils::gpu_eval, Config};

fn image_20k_test() -> () {
    let mut config = Config::default();
    config.size_x = 20000;
    config.size_y = 20000;

    let runtime = tokio::runtime::Builder::new_current_thread().build().unwrap();
    runtime.block_on(async {
        gpu_eval(&config).await.unwrap();
    });

}

fn image_1024_test() -> () {
    let mut config = Config::default();
    config.size_x = 1024;
    config.size_y = 1024;

    let runtime = tokio::runtime::Builder::new_current_thread().build().unwrap();
    runtime.block_on(async {
        gpu_eval(&config).await.unwrap();
    });

}


fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("Generate 20kpx", |b| b.iter(|| image_20k_test()));
    c.bench_function("Generate 1024pk", |b| b.iter(|| image_1024_test()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
