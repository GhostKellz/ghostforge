# 📖 `forge` Command Reference

> This document outlines all current and planned subcommands for the `forge` CLI, part of the GhostForge system. Commands are grouped by usage category for clarity and future extensibility.

---

## 🔹 Core Build Commands

| Command         | Description                                                   |
| --------------- | ------------------------------------------------------------- |
| `forge build`   | Build the current package using `forge.lua`, `forge.toml`, `PKGBUILD`, or auto-detect Rust projects (zero-config) |
| `forge install` | Install the built package or Rust binary into the system (enforces signature verification if enabled) |
| `forge clean`   | Remove temporary build artifacts and cache                    |
| `forge info`    | Display metadata and parsed manifest info                     |

---

## 🔹 Project & Metadata Utilities

| Command        | Description                                                 |
| -------------- | ----------------------------------------------------------- |
| `forge init`   | Generate a starter `forge.toml` file                        |
| `forge lint`   | Validate the syntax and structure of a manifest or PKGBUILD (array support) |
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
| `forge test`  | Run tests included in the package or manifest script       |
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

---

## 🦀 Zero-Config Rust Project Support

- If run in a directory with `Cargo.toml` and no manifest, `forge build` will:
  - Build with `cargo build --release`
  - Auto-detect the binary name
  - Prompt to install to `/usr/bin/` (with `sudo` if needed)
- No manifest needed for simple Rust projects!

## 📝 Advanced: Lua Manifest (forge.lua)

- For advanced users, drop a `forge.lua` in your project for full scripting control.
- Example:

```lua
package = {
  name = "ghostscan",
  version = "0.1.0",
  build = function()
    os.execute("cargo build --release")
  end,
  install = function()
    os.execute("install -Dm755 target/release/ghostscan /usr/bin/ghostscan")
  end
}
```

See DOCS.md for more details.
