set -ex

CROSS_VERSION=0.2.0

main() {
  if [ $TRAVIS_OS_NAME = linux ]; then
    # add x11 capable docker images for linux
    docker build -t kneelawk/cross-custom:i686-unknown-linux-gnu-$CROSS_VERSION docker/i686-unknown-linux-gnu
    docker build -t kneelawk/cross-custom:x86_64-unknown-linux-gnu-$CROSS_VERSION docker/x86_64-unknown-linux-gnu
  fi
}

main
