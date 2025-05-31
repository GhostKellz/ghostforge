# 📖 `forge` Command Reference

> This document outlines all current and planned subcommands for the `forge` CLI, part of the GhostForge system. Commands are grouped by usage category for clarity and future extensibility.

---

## 🔹 Core Build Commands

| Command         | Description                                                   |
| --------------- | ------------------------------------------------------------- |
| `forge build`   | Build the current package using `PKGBUILD` or `ghostpkg.toml` |
| `forge install` | Install the built package into the system                     |
| `forge clean`   | Remove temporary build artifacts and cache                    |
| `forge info`    | Display metadata and parsed manifest info                     |

---

## 🔹 Project & Metadata Utilities

| Command        | Description                                                 |
| -------------- | ----------------------------------------------------------- |
| `forge init`   | Generate a starter `ghostpkg.toml` file                     |
| `forge lint`   | Validate the syntax and structure of a manifest or PKGBUILD |
| `forge sign`   | Sign the built package using GPG                            |
| `forge verify` | Verify signatures and checksums of a package                |

---

## 🔹 Package Distribution

| Command         | Description                                              |
| --------------- | -------------------------------------------------------- |
| `forge publish` | Upload a package to a repo or mirror                     |
| `forge sync`    | Sync with a local or remote pacman-compatible repository |
| `forge submit`  | Submit to the AUR (future)                               |
| `forge upload`  | Push packages to object storage or remote endpoint       |

---

## 🔹 Developer & Testing Tools

| Command       | Description                                                |
| ------------- | ---------------------------------------------------------- |
| `forge test`  | Run tests included in the package or ghostpkg.toml script  |
| `forge audit` | Scan for insecure dependencies, license conflicts, or CVEs |
| `forge deps`  | View and analyze build/runtime dependencies                |

---

## 🔹 Experimental / Advanced (Planned)

| Command         | Description                                                 |
| --------------- | ----------------------------------------------------------- |
| `forge diff`    | Show changes between package versions or metadata revisions |
| `forge rebuild` | Rebuild all packages that depend on this one                |
| `forge sandbox` | Execute builds in isolated, reproducible environments       |
| `forge watch`   | Automatically rebuild on file changes (like `cargo-watch`)  |
