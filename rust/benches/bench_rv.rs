#[macro_use]
extern crate criterion;
extern crate lather;
use criterion::Criterion;
use lather::{linspace, Bounds, Simulation, SpotConfig};

/*
fn observe_flux(b: &mut Bencher) {
    let mut sim = Simulation::sun();
    sim.add_spot(&SpotConfig {
        latitude: 30.0,
        longitude: 180.0,
        fill_factor: 0.01,
        plage: false,
    });
    let time: Vec<f64> = linspace(0.0, 25.05, 100).collect();
    b.iter(|| sim.observe_flux(&time, Bounds::new(4000e-10, 5000e-10)));
}
*/

fn observe_rv(c: &mut Criterion) {
    c.bench_function("observe 100", |b| {
        let mut sim = Simulation::sun();
        sim.add_spot(&SpotConfig {
            latitude: 30.0,
            longitude: 180.0,
            fill_factor: 0.01,
            plage: false,
        });

        let time: Vec<f64> = linspace(0.0, 25.05, 100).collect();

        b.iter(|| sim.observe_rv(&time, Bounds::new(4000e-10, 5000e-10)))
    });
}

criterion_group!(benches, observe_rv);
criterion_main!(benches);
