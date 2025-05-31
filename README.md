# 👻 GhostForge

![Build Status](https://img.shields.io/github/actions/workflow/status/ghostkellz/ghostforge/ci.yml?branch=main)
![License](https://img.shields.io/github/license/ghostkellz/ghostforge)
![Crates.io](https://img.shields.io/crates/v/ghostforge)
![Rust Version](https://img.shields.io/badge/rust-1.78+-blueviolet)
![Arch Linux](https://img.shields.io/badge/archlinux-compatible-blue)

> A blazing-fast, modern, Rust-based replacement for `makepkg` and `PKGBUILD` workflows — designed for Arch Linux users who want performance, clarity, and extensibility.

---

## 🚀 Features

* 🔧 Drop-in replacement for `makepkg`
* 📦 Backward-compatible with existing `PKGBUILD` files
* 🛠️ `ghostpkg.toml` format for modern declarative packaging
* ⚡ Written in pure Rust with zero runtime dependencies
* 🔍 Linting and static validation of build metadata
* 🔐 Optional signing and hash verification
* 🧱 Pluggable build backends (e.g., `cargo`, `cmake`, `just`)
* 📤 Integration-ready with `pacman`, custom repos, or CI pipelines
* Now supports full PKGBUILD array parsing (depends, makedepends, optdepends)
* Parallel build support for subdirectories
* Enforces signature verification before install
* Colorized output and improved error messages
* See DOCS.md for advanced usage, hooks, and plugin system

---

## 🧰 Installation

### From Source

```bash
cargo install ghostforge
```

### From Arch Package (coming soon)

```bash
pacman -U ghostforge-x.y.z.pkg.tar.zst
```

---

## 🛠️ Usage

### Standard PKGBUILD

```bash
ghostforge build
```

### With ghostpkg.toml

```bash
ghostforge build -f ghostpkg.toml
```

### Other Commands

```bash
ghostforge lint         # Validate a PKGBUILD or ghostpkg.toml
ghostforge install      # Install built package to system
ghostforge sign         # Sign a package using GPG
ghostforge publish      # Push to a local repo or remote mirror
```

---

## 📁 Example ghostpkg.toml

```toml
name = "ghostctl"
version = "0.3.0"
author = "CK Technology LLC"
license = "MIT"
build = "cargo build --release"
install = "install -Dm755 target/release/ghostctl /usr/bin/ghostctl"
source = "https://github.com/ghostkellz/ghostctl/archive/v0.3.0.tar.gz"
checksum = "sha256:abcd1234..."
```

---

## 📚 Documentation

* Full CLI Reference: [`ghostforge --help`](https://github.com/ghostkellz/ghostforge/wiki/CLI-Reference)
* [ghostpkg.toml Specification](https://github.com/ghostkellz/ghostforge/wiki/ghostpkg.toml-Spec)
* Migration Guide from `makepkg`: [Read Here](https://github.com/ghostkellz/ghostforge/wiki/Migrating-from-Makepkg)
* See [`DOCS.md`](DOCS.md) for advanced usage, hooks, and plugin system.

---

## 🤝 Contributing

We welcome pull requests, issues, and feature suggestions. See [`CONTRIBUTING.md`](CONTRIBUTING.md) for details.

---

## 📜 License

MIT License © 2025 CK Technology LLC — See [`LICENSE`](LICENSE) for details.

---

## 👻 Project Status

GhostForge is under active development. It is stable for basic use and growing toward full Arch ecosystem integration.

* [x] PKGBUILD parsing
* [x] ghostpkg.toml support
* [x] Build & install
* [ ] Signed package support
* [ ] `pacman` repo sync
* [ ] AUR helper integration
