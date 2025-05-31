# GhostForge Advanced Usage & Architecture

## Advanced PKGBUILD Compatibility
- Full support for PKGBUILD arrays: depends, makedepends, optdepends
- Robust variable extraction and validation

## Performance Optimizations
- Parallel build for subdirectories
- Source and artifact caching (planned)

## Security Enhancements
- Enforced hash/signature verification before install
- Sandboxed builds (bubblewrap/chroot)
- Privilege dropping (planned)

## User Experience & Extensibility
- Colorized output and clear error messages
- Plugin/hook system (planned)
- Migration tools for PKGBUILD → ghostpkg.toml (planned)

See README.md and COMMANDS.md for CLI usage.
