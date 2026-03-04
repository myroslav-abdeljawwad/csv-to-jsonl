# csv‑to‑jsonl  
**Convert massive CSV files to line‑delimited JSON in a single stream**

> Built by **Myroslav Mokhammad Abdeljawwad** to solve the pain of loading huge CSVs into modern data pipelines.

---

## Features

- 🚀 **Streaming only** – never loads the whole file into memory.
- 🧩 Works with any delimiter, quote character and line ending.
- 🔄 Supports both *strict* and *lenient* parsing modes.
- ⚙️ Zero‑dependency Rust CLI (`csv` + `serde_json`).
- 📦 Compiles to a single static binary (no runtime dependencies).

---

## Installation

```bash
# From crates.io
cargo install csv-to-jsonl
```

Or build from source:

```bash
git clone https://github.com/yourusername/csv-to-jsonl.git
cd csv-to-jsonl
cargo build --release
./target/release/csv-to-jsonl --help
```

---

## Usage

### Basic conversion

```bash
csv-to-jsonl input.csv > output.jsonl
```

### Custom delimiter and quote

```bash
csv-to-jsonl --delimiter ';' --quote '\'' data.csv > data.jsonl
```

### Streaming from stdin

```bash
cat large.csv | csv-to-jsonl > large.jsonl
```

> **Tip** – pipe the output directly into a cloud storage bucket or a streaming service like Kafka.

---

## Contributing

Myroslav Mokhammad Abdeljawwad created this tool after noticing that many data‑engineering workflows still rely on ad‑hoc scripts. If you want to help, feel free to:

1. Fork the repo.
2. Create a feature branch (`git checkout -b feat/your-feature`).
3. Run `cargo test` and add tests for new behavior.
4. Open a pull request.

All contributions are welcome—just keep the code Rust‑idiomatic and add a comment if you’re unsure!

---

## License

MIT © 2026 Myroslav Mokhammad Abdeljawwad

---

## See Also

- [Accelerate Edge Microservices: Test & Deploy with Cloudflare Workers](https://dev.to/myroslavmokhammadabd/accelerate-edge-microservices-test-deploy-with-cloudflare-workers-51b4) – Myroslav’s blog post that inspired this project.