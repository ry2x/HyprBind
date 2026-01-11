# Maintainer: ry2x <45420571+ry2x@users.noreply.github.com>

pkgname='hyprbind'
_pkgname='HyprBindViewer'
pkgver='0.1.1'
pkgrel=1
pkgdesc='A GUI to display Hyprland keybindings'
arch=('x86_64')
url=''
license=('MIT')
depends=('hyprland')
makedepends=('cargo' 'git')
source=("$pkgname::git+file://$PWD")
sha256sums=('SKIP')

build() {
    cd "$pkgname"
    CARGO_TARGET_DIR=target cargo build --release --locked
}

package() {
    cd "$pkgname"
    install -Dm755 "target/release/$_pkgname" "$pkgdir/usr/bin/$pkgname"
    install -Dm644 "assets/logo_hyprbind.png" "$pkgdir/usr/share/pixmaps/$pkgname.png"
    install -Dm644 "hyprbind.desktop" "$pkgdir/usr/share/applications/$pkgname.desktop"
}
