# Maintainer: Your Name <youremail@domain.com>
pkgname=tetris-cli-git
pkgver=1.0
pkgrel=1
epoch=
pkgdesc="A tetris game that runs in a terminal"
arch=('any')
url="https://www.github.com/blueOkiris/tetris-cli"
license=('GPL3')
source=('$pkgname-$pkgver.tar.gz')

build() {
	cd "$pkgname-$pkgver"
	make tetris-cli
}

package() {
	cd "$pkgname-$pkgver"
	make install
}
