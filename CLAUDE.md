# denso — Wireless ADB Manager

mDNS ADB discovery, QR code pairing, auto-reconnect. Consumes AdbTransport, UsbEnumerator traits.

## Build & Test

```bash
cargo build
cargo test
cargo run -- discover
cargo run -- pair
cargo run -- connect
```

## Conventions

- Edition 2024, Rust 1.91.0+, MIT, clippy pedantic
- Release: codegen-units=1, lto=true, opt-level="z", strip=true
