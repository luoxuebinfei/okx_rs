use criterion::{black_box, criterion_group, criterion_main, Criterion};
use okx_core::{Credentials, Signer};

fn bench_signer(c: &mut Criterion) {
    let credentials = Credentials::new("api_key", "secret_key", "passphrase");
    let signer = Signer::new(credentials);

    let request_path = "/api/v5/account/balance?ccy=BTC";
    let body = r#"{"instId":"BTC-USDT-SWAP","tdMode":"cross","side":"buy","sz":"1"}"#;
    let timestamp = "2024-01-01T00:00:00.000Z";

    c.bench_function("pre_hash", |b| {
        b.iter(|| {
            Signer::pre_hash(
                black_box(timestamp),
                black_box("POST"),
                black_box(request_path),
                black_box(body),
            )
        })
    });

    let pre_hash = Signer::pre_hash(timestamp, "POST", request_path, body);

    c.bench_function("sign", |b| {
        b.iter(|| Signer::sign(black_box(&pre_hash), black_box("secret_key")))
    });

    c.bench_function("generate_headers", |b| {
        b.iter(|| {
            signer.generate_headers(
                black_box("POST"),
                black_box(request_path),
                black_box(body),
                black_box(true),
            )
        })
    });
}

criterion_group!(benches, bench_signer);
criterion_main!(benches);
