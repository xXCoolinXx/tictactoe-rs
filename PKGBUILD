# Maintainer: xXCoolinXx collin.t.francel@gmail.com
#
# This PKGBUILD was generated by `cargo aur`: https://crates.io/crates/cargo-aur

pkgname=xo-rs-bin
pkgver=0.5.1
pkgrel=1
pkgdesc="TicTacToe terminal game with Rust"
url="https://github.com/xXCoolinXx/xo-rs"
license=("GPL-3.0-or-later")
arch=("x86_64")
provides=("xo-rs")
conflicts=("xo-rs")
source=("https://github.com/xXCoolinXx/xo-rs/releases/download/v$pkgver/xo-rs-$pkgver-x86_64.tar.gz")
sha256sums=("eefe26abcdcc9b7f1164f6d49dc05c906b100df7fdafc9ee96f106e54f3a8c17")

package() {
    install -Dm755 xo-rs -t "$pkgdir/usr/bin"
    install -Dm644 LICENSE "$pkgdir/usr/share/licenses/$pkgname/LICENSE"
}
