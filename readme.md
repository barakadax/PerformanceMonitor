# Performance monitor
Monitor another process in runtime,
CPU, RAM & Disk,

local is to run from CLI,
service is to run as binary with VS code extension,

OS supported: Windows & Unix based

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
    <li>Use sysinfo to get info for everything</li>
    <li>Memory heap vs stack and usage</li>
    <li>Find replacement for sysinfo to get network information, best option `pcap` for win and mac works out of the box for unix based needs to install `libpcap` & run with `CAP_NET_RAW` privilege</li>
    <li>Find replacement for sysinfo to get GPU information</li>
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
