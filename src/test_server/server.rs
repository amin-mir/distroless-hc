use std::env;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

use tokio::signal::unix::{signal, SignalKind};
use warp::http::status::StatusCode;
use warp::Filter;

#[tokio::main]
async fn main() {
    let port: u16 = env::var("PORT")
        .unwrap_or("3030".to_string())
        .parse()
        .expect("PORT should be a valid number");

    let fail_count: usize = env::var("FAIL_COUNT")
        .unwrap_or("0".to_string())
        .parse()
        .expect("FAIL_COUNT should be a valid number");

    println!(
        "Running the sever on port {} with fail count {}",
        port, fail_count
    );

    let num_reqs = Arc::new(AtomicUsize::new(0));

    let health_check = warp::path("healthcheck").map(move || {
        println!("Received a request");
        let reqs = num_reqs.fetch_add(1, Ordering::Relaxed) + 1;
        if reqs >= fail_count {
            StatusCode::ACCEPTED
        } else {
            StatusCode::INTERNAL_SERVER_ERROR
        }
    });

    let shutdown_signal = async {
        let mut int_sig = signal(SignalKind::interrupt()).unwrap();
        let mut term_sig = signal(SignalKind::terminate()).unwrap();

        tokio::select! {
            _ = int_sig.recv() => {}
            _ = term_sig.recv() => {}
        }
    };

    let (_, server) = warp::serve(health_check)
        .bind_with_graceful_shutdown(([0, 0, 0, 0], port), shutdown_signal);

    server.await
}
