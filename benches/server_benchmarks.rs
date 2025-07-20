use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rust_https_server::http::{Request, Response};
use rust_https_server::router::Router;
use http::{Method, Uri, Version};

fn benchmark_router_creation(c: &mut Criterion) {
    c.bench_function("router_creation", |b| {
        b.iter(|| {
            let mut router = Router::new();
            router
                .get("/", |_| Ok(Response::ok().with_text("Hello")))
                .get("/user-agent", |_| Ok(Response::ok().with_text("User-Agent")))
                .get("/echo/{param}", |_| Ok(Response::ok().with_text("Echo")));
            black_box(router);
        });
    });
}

fn benchmark_response_creation(c: &mut Criterion) {
    c.bench_function("response_creation", |b| {
        b.iter(|| {
            let response = Response::ok()
                .with_content_type("text/plain")
                .with_text("Hello, World!");
            black_box(response);
        });
    });
}

fn benchmark_request_creation(c: &mut Criterion) {
    c.bench_function("request_creation", |b| {
        b.iter(|| {
            let uri = "http://localhost:4221/test".parse::<Uri>().unwrap();
            let request = Request::new(Method::GET, uri, Version::HTTP_11);
            black_box(request);
        });
    });
}

fn benchmark_mime_type_detection(c: &mut Criterion) {
    c.bench_function("mime_type_detection", |b| {
        b.iter(|| {
            let mime_type = rust_https_server::utils::get_mime_type("test.html");
            black_box(mime_type);
        });
    });
}

criterion_group!(
    benches,
    benchmark_router_creation,
    benchmark_response_creation,
    benchmark_request_creation,
    benchmark_mime_type_detection
);
criterion_main!(benches); 