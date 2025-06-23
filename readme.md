# Performance monitor
Monitor another process in runtime,
CPU, RAM, Disk, & Network

local is to run from CLI
service is to run as binary with VS code extension

## Pre steps for developers
<ol>
    <li>Rust minimum version: 1.87.0</li>
    <li> <h3>Environment variables flags:</h3>
        <ul>
            <li>RUST_LOG=debug</li>
        </ul>
    </li>
</ol>

## How to run:
cargo run --bin local python3 a.py

## TODO:
<ul>
    <li>Custom AVG calculator</li>
    <li>Use sysinfo to get info for everything</li>
    <li>Coverage / Unit tests</li>
    <li>Integration with NodeJS TS: <a href="https://github.com/barakadax/PerformaceMonitorVScodeExtension">VS code extension repo</a></li>
    <li>GitHub actions build binary, run tests</li>
</ul>

## commands
cargo run --bin local python3 a.py

cargo run --bin service python3 a.py

cargo fmt

cargo test

cargo bench

cargo build --release --bin local/service

ctrl + c == stop

ctrl + z == zombie

kill -9 process_number == force kill

jobs == see all zombies

fg == return zombie

## Open cases created from working on this project:
<ol>
    <li>https://users.rust-lang.org/t/child-process-never-terminated-unix/130990</li>
    <li>https://github.com/tokio-rs/tracing/issues/3321</li>
    <li>https://github.com/tokio-rs/tracing/issues/3322</li>
</ol>
