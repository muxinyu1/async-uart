# async-uart-driver

## Usage

```toml
# Cargo.toml

[dependencies]

# when testing in qemu emulator
async-uart = { git = "https://github.com/muxinyu1/async-uart", features = ["board_qemu"]}

# when testing on a board
async-uart = { git = "https://github.com/muxinyu1/async-uart", features = ["board_lrv"]}

# the default feature is "board_qemu"
async-uart = { git = "https://github.com/muxinyu1/async-uart"}
```