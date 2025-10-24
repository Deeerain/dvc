pkgname=dvc
pkgver=0.0.1
pkgrel=1
pkgdesc="Simple volume control panel"
arch=('x86_64' 'aarch64')
url="https://github.com/Deeerain/dvc"
depends=('glibc')
makedepends=('cargo' 'git')
source=("git+https://github.com/deeerain/dvc.git#tag=v$pkgver")
sha256sums=('SKIP')

prepare() {
  cd "$pkgname"
  cargo fetch --locked --target "$CARCH-unknown-linux-gnu"
}

build() {
  cd "$pkgname"
  export RUSTUP_TOOLCHAIN=stable
  export CARGO_TARGET_DIR=target
  cargo build --frozen --release --all-features
}

check() {
  cd "$pkgname"
  export RUSTUP_TOOLCHAIN=stable
  cargo test --frozen --all-features
}

package() {
  cd "$pkgname"
  install -Dm0755 -t "$pkgdir/usr/bin/" "target/release/$pkgname"
}
