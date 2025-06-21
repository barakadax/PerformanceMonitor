# performance
Monitor another process in runtime,
CPU, RAM, Disk, & Network

local is to run from CLI
service is to run as binary with VS code extension

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
