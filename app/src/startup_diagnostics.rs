use std::sync::LazyLock;

use instant::Instant;

static STARTED: LazyLock<Instant> = LazyLock::new(Instant::now);

pub fn start() {
    LazyLock::force(&STARTED);
}

pub fn elapsed_ms() -> f64 {
    STARTED.elapsed().as_secs_f64() * 1000.0
}
