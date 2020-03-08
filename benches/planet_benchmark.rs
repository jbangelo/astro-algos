use astro_algos::{planets::Planet, time::JD};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn mercury_position_benchmark(c: &mut Criterion) {
    c.bench_function("mercury position", |b| {
        b.iter(|| Planet::Mercury.get_location(black_box(&JD::from(2268920.0))))
    });
}

pub fn venus_position_benchmark(c: &mut Criterion) {
    c.bench_function("venus position", |b| {
        b.iter(|| Planet::Venus.get_location(black_box(&JD::from(2268920.0))))
    });
}

pub fn earth_position_benchmark(c: &mut Criterion) {
    c.bench_function("earth position", |b| {
        b.iter(|| Planet::Earth.get_location(black_box(&JD::from(2268920.0))))
    });
}

pub fn mars_position_benchmark(c: &mut Criterion) {
    c.bench_function("mars position", |b| {
        b.iter(|| Planet::Mars.get_location(black_box(&JD::from(2268920.0))))
    });
}

pub fn jupiter_position_benchmark(c: &mut Criterion) {
    c.bench_function("jupiter position", |b| {
        b.iter(|| Planet::Jupiter.get_location(black_box(&JD::from(2268920.0))))
    });
}

pub fn saturn_position_benchmark(c: &mut Criterion) {
    c.bench_function("saturn position", |b| {
        b.iter(|| Planet::Saturn.get_location(black_box(&JD::from(2268920.0))))
    });
}

pub fn uranus_position_benchmark(c: &mut Criterion) {
    c.bench_function("uranus position", |b| {
        b.iter(|| Planet::Uranus.get_location(black_box(&JD::from(2268920.0))))
    });
}

pub fn neptune_position_benchmark(c: &mut Criterion) {
    c.bench_function("neptune position", |b| {
        b.iter(|| Planet::Neptune.get_location(black_box(&JD::from(2268920.0))))
    });
}

criterion_group!(
    benches,
    mercury_position_benchmark,
    venus_position_benchmark,
    earth_position_benchmark,
    mars_position_benchmark,
    jupiter_position_benchmark,
    saturn_position_benchmark,
    uranus_position_benchmark,
    neptune_position_benchmark
);
criterion_main!(benches);
