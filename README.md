# Mulity-Snake-stm32f7-Rust
Multiplayer Snake for stm32f7 written in Rust

**PLS**:  
copy either `.gdbinitv7` or `.gdbinitv8` to `.gdbinit` depending on your gdb version. (or make symlink).

```bash
cp .gdbinitv7 .gdbinit
```

## Build + Run
On Linux/Mac:

```bash
# build: (cross-compile for stm)
RUST_TARGET_PATH=$(pwd) xargo build
# run
sh gdb.sh
# wait for it
semihosting-enable
c # continue
# CTRL+C
# q (quit)
```
