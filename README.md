# cargo-cargofmt

A CLI tool to format Cargo configuration files (`Cargo.toml`, `.cargo/config.toml`, `rustfmt.toml`, `clippy.toml`) with consistent structure, ordering, and style.

---

## 🚀 Motivation

While `cargo fmt` formats Rust source code, there is no official tool to format Cargo configuration files.

This leads to:

* Inconsistent formatting across projects
* Reduced readability
* Noisy diffs in version control

**cargo-cargofmt** solves this by providing a standardized formatter for TOML-based Cargo configuration files.

---

## ✨ Features

### 📦 Section Ordering

* `[package]` section is placed first
* `[dependencies]` section is placed last

### 🔤 Dependency Sorting

* Dependencies are sorted alphabetically
* Ensures deterministic and clean ordering

### 🎯 Formatting Normalization

* Consistent spacing (`key = value`)
* Standardized double-quoted values

### 🧠 Smart Formatting

* Built using `toml_edit`
* Preserves structure while applying transformations

### ⚡ CLI Support

* Format files directly from terminal
* Simple and fast execution

---

## 🛠️ Installation

```bash
git clone https://github.com/THARUN-BART/cargo-cargofmt.git
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
│   ├── formatter.rs   # Formatting rules and transformations
```

---

## 🧠 Implementation Details

* Uses `toml_edit` for parsing and modifying TOML
* Applies deterministic formatting rules
* Reconstructs sections for consistent ordering
* Ensures minimal disruption to valid TOML structure

---

## ⚠️ Edge Cases Handled

* Missing sections are handled gracefully
* Preserves valid TOML structure
* Avoids breaking unrelated sections

---

## ⚠️ Current Limitations

* No support yet for:

  * `[dev-dependencies]`
  * `[build-dependencies]`
* Limited comment preservation
* Formatting rules are not yet configurable

---

## 🚧 Future Work

### Core Enhancements

* Support additional dependency sections
* Improve comment preservation

### Usability Improvements

* Configurable formatting rules
* CLI flags and options

### Integration

* Cargo subcommand support (`cargo cargofmt`)
* Wider Rust ecosystem integration

---

## 📌 GSoC Context

This project is a working prototype for improving Cargo configuration formatting as part of a Google Summer of Code proposal.

It demonstrates:

* TOML parsing and transformation using `toml_edit`
* Deterministic formatting rules
* CLI-based formatting workflow

The project will be extended to include configurable formatting, broader file support, and deeper integration with Cargo.

---

## 🤝 Contribution

Contributions are welcome!
Feel free to open issues or submit pull requests.

---

## 📄 License

MIT License
