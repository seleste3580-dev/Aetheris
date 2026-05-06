<div align="center">
  <img src="./aetheris/demo.svg" alt="Aetheris Scanner" width="800"/>

  # Aetheris
  
  **Blazing-Fast Asynchronous Attack Surface Mapper**

  [![Rust](https://img.shields.io/badge/Language-Rust-orange.svg?style=for-the-badge&logo=rust)](#)
  [![License](https://img.shields.io/badge/License-MIT-blue.svg?style=for-the-badge)](#)
  [![Status](https://img.shields.io/badge/Status-Enterprise_Ready-brightgreen.svg?style=for-the-badge)](#)
</div>

<br/>

Aetheris is a next-generation port scanning engine built from the ground up in Rust. Designed to replace legacy synchronous tools, it leverages the `Tokio` async runtime to multiplex thousands of raw socket connections simultaneously, making it capable of scanning entire subnets in seconds.

## 🚀 Features

- **Asynchronous Core:** Non-blocking I/O multiplexing handles 5000+ connections seamlessly.
- **Dynamic Terminal UX:** Beautiful, real-time progress bars with intelligent time estimation.
- **Enterprise Integration:** Natively exports to clean, structured JSON formats.
- **Cyberpunk Dark Theme:** Engineered for hackers, prioritizing low-light readability and slick aesthetics.

## ⚙️ Installation

Ensure you have [Rust & Cargo](https://rustup.rs/) installed.

```bash
git clone https://github.com/seleste-technologies/aetheris.git
cd aetheris
cargo build --release
```

The compiled binary will be available at `./target/release/aetheris`.

## 💻 Usage

```bash
# Basic Scan (Scans all 65,535 ports by default)
./target/release/aetheris 192.168.1.1

# Aggressive Custom Scan (Faster concurrency, specific ports)
./target/release/aetheris 10.0.0.5 --ports 1-1024 --concurrency 10000

# Export to JSON for Platform Integration
./target/release/aetheris 127.0.0.1 --output result.json
```

---
<div align="center">
  <sub>Built with ❤️ by <b>Seleste Technologies</b></sub>
</div>
