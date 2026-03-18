# cargo-cargofmt

A CLI tool to format Cargo configuration files (`Cargo.toml`, `.cargo/config.toml`, `rustfmt.toml`, `clippy.toml`) with consistent structure, ordering, and style.

---

## 🚀 Motivation

While `cargo fmt` formats Rust source code, there is no official tool to format Cargo configuration files.
This results in inconsistent formatting, reduced readability, and unnecessary diffs in version control.

**cargo-cargofmt** aims to provide a standardized formatter for TOML-based Cargo configuration files.

---

## ✨ Features

* 📦 Section ordering

  * `[package]` is placed first
  * `[dependencies]` is placed last

* 🔤 Dependency sorting

  * Dependencies are sorted alphabetically

* 🎯 Formatting normalization

  * Ensures consistent spacing (`key = value`)
  * Converts values to standard double quotes (`" "`)

* 🧠 Smart formatting

  * Built using `toml_edit`
  * Preserves structure and supports formatting transformations

* ⚡ CLI support

  * Format files directly from the terminal

---

## 🛠️ Installation

```bash
git clone https://github.com/<your-username>/cargo-cargofmt.git
cd cargo-cargofmt
cargo build
```

---

## ▶️ Usage

```bash
cargo run Cargo.toml
```

or

```bash
./target/debug/cargo-cargofmt Cargo.toml
```

---

## 🧪 Example

### Before

```toml
[dependencies]
toml_edit='0.25.5'
serde="1.0.228"
toml='1.0.7'

[package]
edition="2024"
name="cargo-cargofmt-clone"
version="0.1.0"
```

---

### After

```toml
[package]
edition = "2024"
name = "cargo-cargofmt-clone"
version = "0.1.0"

[dependencies]
serde = "1.0.228"
toml = "1.0.7"
toml_edit = "0.25.5"
```

---

## 🏗️ Project Structure

```
cargo-cargofmt/
├── Cargo.toml
├── src/
│   ├── main.rs        # CLI entry point
│   ├── parser.rs      # TOML parsing logic
│   ├── formatter.rs   # Formatting rules (ordering, sorting, normalization)
```

---

## 🧠 Implementation Details

* Uses `toml_edit` for parsing and editing TOML
* Reconstructs the document to enforce consistent section ordering
* Applies deterministic sorting for dependencies
* Normalizes spacing and quotation styles

---

## 🚧 Future Work

* Support for:

  * `[dev-dependencies]`
  * `[build-dependencies]`
* Configurable formatting rules
* Integration as a Cargo subcommand (`cargo cargofmt`)
* Formatting support for `rustfmt.toml` and `clippy.toml`
* Advanced formatting (alignment, comments preservation improvements)

---

## 🤝 Contribution

Contributions are welcome!
Feel free to open issues or submit pull requests.

---

## 📌 GSoC Context

This project is developed as a prototype for Google Summer of Code (GSoC) to improve formatting support in the Cargo ecosystem.

---

## 📄 License

MIT License
