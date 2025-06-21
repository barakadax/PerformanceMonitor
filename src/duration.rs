use std::time::Duration;

const NANO: u128 = 1_000;
const MICRO: u128 = 1_000_000;
const MILLI: u128 = 1_000_000_000;
const SECOND: u128 = 60_000_000_000;
const MINUTE: u128 = 3_600_000_000_000;

pub fn format_duration(duration: Duration) -> String {
    let nanos: u128 = duration.as_nanos();
    if nanos < NANO {
        format!("{}ns", nanos)
    } else if nanos < MICRO {
        format!("{:.3}µs", nanos as f64 / NANO as f64)
    } else if nanos < MILLI {
        format!("{:.3}ms", nanos as f64 / MICRO as f64)
    } else if nanos < SECOND {
        format!("{:.3}s", nanos as f64 / MILLI as f64)
    } else if nanos < MINUTE {
        format!("{:.3}min", nanos as f64 / SECOND as f64)
    } else {
        format!("{:.3}h", nanos as f64 / MINUTE as f64)
    }
}
