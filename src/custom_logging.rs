pub fn init_logging() {
    use tracing_subscriber::{EnvFilter, fmt, prelude::*};

    tracing_subscriber::registry()
        .with(EnvFilter::from_default_env())
        .with(
            fmt::layer().event_format(
                fmt::format()
                    .json()
                    .with_level(true)
                    .with_thread_ids(true)
                    .with_file(true)
                    .with_line_number(true)
                    .with_target(false)
                    .flatten_event(true),
            ),
        )
        .init();
}

#[macro_export]
macro_rules! log_generic {
    ($level:ident) => {
        $level!(pid = std::process::id());
    };

    ($level:ident, $($key:ident = $val:expr),+ , $fmt:literal $(, $arg:expr)* $(,)?) => {
        $level!(pid = std::process::id(), $($key = $val,)* msg = format_args!($fmt $(, $arg)*));
    };

    ($level:ident, $fmt:literal $(, $arg:expr)* $(,)?) => {
        $level!(pid = std::process::id(), msg = format_args!($fmt $(, $arg)*));
    };
}

#[macro_export]
macro_rules! log_info {
    () => {
        $crate::log_generic!(info);
    };
    ($($arg:tt)+) => {
        $crate::log_generic!(info, $($arg)+);
    };
}

#[macro_export]
macro_rules! log_warn {
    () => {
        $crate::log_generic!(warn);
    };
    ($($arg:tt)+) => {
        $crate::log_generic!(warn, $($arg)+);
    };
}

#[macro_export]
macro_rules! log_debug {
    () => {
        $crate::log_generic!(debug);
    };
    ($($arg:tt)+) => {
        $crate::log_generic!(debug, $($arg)+);
    };
}

#[macro_export]
macro_rules! log_error {
    () => {
        $crate::log_generic!(error);
    };
    ($($arg:tt)+) => {
        $crate::log_generic!(error, $($arg)+);
    };
}

#[macro_export]
macro_rules! log_trace {
    () => {
        $crate::log_generic!(trace);
    };
    ($($arg:tt)+) => {
        $crate::log_generic!(trace, $($arg)+);
    };
}
