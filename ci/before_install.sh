set -ex

main() {
  if [ $TRAVIS_OS_NAME = linux ]; then
    pkg-config --libs x11
    dpkg -S /usr/lib/pkgconfig/x11.pc
    dpkg -S /usr/lib/x86_64-linux-gnu/pkgconfig/x11.pc
  fi
}

main
