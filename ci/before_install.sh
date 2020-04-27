set -ex

main() {
  if [ $TRAVIS_OS_NAME = linux ]; then
    sudo apt-get update
    sudo apt-get -y install libx11-dev
  fi
}

main
