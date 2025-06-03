# Maintainer: Your Name <youremail@example.com>

pkgname=ghostforge
gitname=ghostforge
gitver=0.1.0
pkgver=0.1.0
pkgrel=1
pkgdesc="A blazing-fast, modern, Rust-based replacement for makepkg and PKGBUILD workflows. Now with zero-config Rust support and Lua manifest support."
arch=('x86_64')
url="https://github.com/ghostkellz/ghostforge"
license=('MIT')
depends=('rust' 'cargo' 'lua')
makedepends=('git')
source=("git+https://github.com/ghostkellz/ghostforge.git#tag=v${pkgver}")
b2sums=('SKIP')

build() {
  cd "$srcdir/$gitname"
  cargo build --release --locked
}

package() {
  cd "$srcdir/$gitname"
  install -Dm755 target/release/ghostforge "$pkgdir/usr/bin/ghostforge"
}
