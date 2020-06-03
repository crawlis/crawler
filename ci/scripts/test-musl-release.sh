 
#!/bin/bash

# Usage: ./musl-test-release.sh <MODULE_NAME>
#
# Most parts of this script are comming from:
# https://github.com/emk/rust-musl-builder/blob/master/examples/build-release
#
# Called by `.travis.yml` to build release binaries. We use
# ekidd/rust-musl-builder to make the Linux binaries so that we can run
# them unchanged on any distro, including tiny distros like Alpine (which
# is heavily used for Docker containers).

docker build -t "$1"-release -f ./ci/dockerfiles/musl-tester-release.Dockerfile .
docker run -it --name "$1" "$1"-release
docker rm "$1"
docker rmi "$1"-release
       