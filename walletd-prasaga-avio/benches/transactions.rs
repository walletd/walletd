use criterion::{black_box, criterion_group, criterion_main, Criterion};
use walletd_prasaga_avio::PrasagaAvioKeypair;

fn benchmark_keypair_generation(c: &mut Criterion) {
    c.bench_function("keypair from seed", |b| {
        let seed = b"test seed for prasaga avio chain integration!!!";
        b.iter(|| {
            let keypair =
                PrasagaAvioKeypair::from_seed(black_box(seed), black_box("m/44'/9000'/0'/0/0"));
            black_box(keypair)
        });
    });
}

fn benchmark_signing(c: &mut Criterion) {
    let seed = b"test seed for prasaga avio chain integration!!!";
    let keypair = PrasagaAvioKeypair::from_seed(seed, "m/44'/9000'/0'/0/0").unwrap();
    let message = b"Hello PraSaga Avio!";

    c.bench_function("sign message", |b| {
        b.iter(|| {
            let signature = keypair.sign(black_box(message));
            black_box(signature)
        });
    });
}

criterion_group!(benches, benchmark_keypair_generation, benchmark_signing);
criterion_main!(benches);
