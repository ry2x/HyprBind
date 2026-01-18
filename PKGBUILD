# Maintainer: ry2x <45420571+ry2x@users.noreply.github.com>

pkgname='hyprbind'
_pkgname='HyprBind'
pkgver='0.1.4'
pkgrel=1
pkgdesc='A GUI to display Hyprland keybindings'
arch=('x86_64' 'aarch64')
url='https://github.com/ry2x/HyprBind'
license=('MIT' 'OFL-1.1')
depends=('hyprland' 'libxkbcommon' 'wayland' 'libglvnd')
makedepends=('cargo' 'git' 'pkgconf')
source=("$pkgname::git+file://$PWD")
#source=("$pkgname::git+https://github.com/ry2x/HyprBind.git#branch=feat/improve-PKGBUILD-and-Cargo.toml")
sha256sums=('SKIP')

prepare() {
    cd "$pkgname"
    export CARGO_HOME="$srcdir/cargo-home"
    cargo fetch --locked --target "$(rustc -vV | sed -n 's/host: //p')"
}

build() {
    cd "$pkgname"
    export CARGO_HOME="$srcdir/cargo-home"
    CARGO_TARGET_DIR=target cargo build --release --locked --offline
}

package() {
    cd "$pkgname"
    
    install -Dm755 "target/release/$_pkgname" "$pkgdir/usr/bin/$pkgname"
    
    install -Dm644 "assets/logo_hyprbind.png" "$pkgdir/usr/share/pixmaps/$pkgname.png"
    install -Dm644 "hyprbind.desktop" "$pkgdir/usr/share/applications/$pkgname.desktop"
    
    install -Dm644 "LICENSE-MIT" "$pkgdir/usr/share/licenses/$pkgname/LICENSE-MIT"
    install -Dm644 "LICENSE-OFL" "$pkgdir/usr/share/licenses/$pkgname/LICENSE-OFL-1.1"
}
