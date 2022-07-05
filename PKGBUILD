# Maintainer: michal <michal[at]tar[dot]black>
# Developer:  michal <michal[at]tar[dot]black>

pkgname="appendage"
pkgver=0.1.1
pkgrel=1
pkgdesc="Append text to files without all of the hassle"
arch=('x86_64')
url="https://github.com/ihatethefrench/appendage"
license=('GPL3')
source=("git+$url")
sha256sums=('SKIP')
makedepends=('cargo' 'git')

prepare() {
    cd "$srcdir/$pkgname"
    cargo fetch --locked --target "$CARCH-unknown-linux-gnu"
}

build() {
    cd "$srcdir/$pkgname"
    export RUSTUP_TOOLCHAIN=stable
    export CARGO_TARGET_DIR=target
    cargo build --frozen --release --all-features
}

package() {
    cd "$srcdir/$pkgname"
    install -Dm0755 -t "$pkgdir/usr/bin" "target/release/apd"
}