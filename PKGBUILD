# Maintainer: Your Name <youremail@domain.com>
pkgname=tetris-cli-git
pkgver=1.0
pkgrel=1
epoch=
pkgdesc="A tetris game that runs in a terminal"
arch=('any')
url="https://www.github.com/blueOkiris/tetris-cli"
license=('GPL3')
source=('git://github.com/blueOkiris/tetris-cli.git')
pre_remove=$pkgname.install

md5sums=('SKIP')

build() {
	cd tetris-cli
	make tetris-cli
}

package() {
	cd tetris-cli
	make install
}

