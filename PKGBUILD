# Maintainer: Christopher Kelley <ckelley@ghostkellz.sh>

pkgname=ghostforge
pkgver=0.2.0
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
  cd "$srcdir/$pkgname"
  cargo build --release
}

package() {
  cd "$srcdir/$pkgname"
  install -Dm755 target/release/ghostforge "$pkgdir/usr/bin/ghostforge"
}
