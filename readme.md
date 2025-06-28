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
    <li>In mac OS you need to run with taskgated permission & have SIP-disabled or just run as root</li>
</ol>

## How to run:
cargo run --bin local python3 a.py

## TODO:
<ul>
    <li>Add types and translation of sizes</li>
    <li>Add log to know where the monitor.json was created</li>
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

Compile command: cargo build --target i686-pc-windows-gnu --bin local/service --release
How to run: ./target/x86_64-unknown-linux-gnu/release/local python3 /home/barakadax/Desktop/codes/RustPref/a.py

Other OS compile commands:
cargo build --target x86_64-pc-windows-gnu --bin local/service --release
cargo build --target x86_64-apple-darwin --bin local/service --release
cargo build --target x86_64-unknown-linux-gnu --bin local/service --release
