# GhostForge Advanced Usage & Architecture

## Advanced PKGBUILD Compatibility
- Full support for PKGBUILD arrays: depends, makedepends, optdepends
- Robust variable extraction and validation

## Zero-Config Rust Project Support
- If run in a directory with `Cargo.toml` and no manifest, `forge build` will:
  - Build with `cargo build --release`
  - Auto-detect the binary name
  - Prompt to install to `/usr/bin/` (with `sudo` if needed)
- No manifest needed for simple Rust projects!

## Lua Manifest (forge.lua)
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

## Performance Optimizations
- Parallel build for subdirectories
- Source and artifact caching

## Security Enhancements
- Enforced hash/signature verification before install
- Sandboxed builds (bubblewrap/chroot)
- Privilege dropping (planned)

## User Experience & Extensibility
- Colorized output and clear error messages
- Plugin/hook system (planned)
- Linting for PKGBUILD and forge.toml
- Real-world usage and troubleshooting tips in README.md

See README.md and COMMANDS.md for CLI usage.
