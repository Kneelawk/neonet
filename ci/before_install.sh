set -ex

main() {
  if [ $TRAVIS_OS_NAME = linux ]; then
    sudo apt-get update -qq
    sudo apt-get -y install libxss-dev
  fi
}

main
