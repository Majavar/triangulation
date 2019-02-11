#[macro_use]
extern crate criterion;

use criterion::{AxisScale, Criterion, ParameterizedBenchmark, PlotConfiguration};
use graph::{Delaunay, Point, Voronoi};
use rand::{rngs::StdRng, Rng, SeedableRng};
use std::iter::repeat_with;

const COUNTS: &[usize] = &[100, 1000, 10_000, 100_000, 1_000_000];

fn bench(c: &mut Criterion) {
    let mut rng = StdRng::seed_from_u64(123456);

    let all_points: Vec<_> = repeat_with(|| rng.gen())
        .map(|(x, y)| Point::new(x, y))
        .take(*COUNTS.last().unwrap())
        .collect();

    let bench = ParameterizedBenchmark::new(
        "voronoi",
        move |b, &&count| {
            let points = all_points[..count].to_vec();
            let delaunay = Delaunay::from(points).unwrap();

            b.iter(|| Voronoi::from(&delaunay))
        },
        COUNTS,
    );

    c.bench(
        "voronoi",
        bench
            .sample_size(20)
            .plot_config(PlotConfiguration::default().summary_scale(AxisScale::Logarithmic)),
    );
}

criterion::criterion_group!(benches, bench);
criterion::criterion_main!(benches);
