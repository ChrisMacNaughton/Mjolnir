if [ -z "$1" ]
  then
    echo "No argument supplied, requires build version"
    exit 1
fi

set -euo pipefail

distro=$1

path=`pwd`
echo "About to launch $distro container"
container="mjolnir-build-$RANDOM"
mkdir -p /tmp/$container

function finish {
    echo "Cleaning up: ($?)!"
    lxc delete -f $container
    rm -rf /tmp/$container
    echo "finished cleaning up"
}
trap finish EXIT

echo "Named container: $container"
lxc launch ubuntu:$distro --ephemeral $container > /dev/null
echo "Launched $container"

echo "Setting up /build"

lxc exec $container -- /bin/sh -c "/bin/mkdir -p /build"
echo "Pushing files into container"
tar --exclude-vcs --exclude=target -zcf - . | lxc exec --verbose $container -- /bin/sh -c "/bin/tar zxf - -C /build"
sleep 5

echo "Installing deps"
packages="libssl-dev protobuf-compiler libprotobuf-dev libsodium-dev liblzma-dev pkg-config"
if [ "$distro" = "trusty" ]
    then
    lxc exec --verbose $container -- /bin/sh -c "add-apt-repository -y ppa:chris-lea/libsodium"
    lxc exec --verbose $container -- /bin/sh -c "add-apt-repository -y ppa:alexhuang/libzmq"
    packages="$packages libzmq"
fi
if [ "$distro" = "xenial" ]
    then
    packages="$packages libzmq5-dev"
fi
lxc exec --verbose $container -- /bin/sh -c "apt-get update -q"
lxc exec --verbose $container -- /bin/sh -c "apt-get install -yq $packages"

echo "About to install rust"
lxc exec --verbose $container -- /bin/sh -c "curl https://sh.rustup.rs -sSf | sh -s -- -y"

echo "Installing cargo-deb"
lxc exec --verbose $container -- /bin/sh -c "/root/.cargo/bin/cargo install cargo-deb"

echo "Building deb"
lxc exec --verbose $container -- /bin/sh -c "cd /build; /root/.cargo/bin/cargo build --release --all"
lxc exec --verbose $container -- /bin/sh -c "cd /build; /root/.cargo/bin/cargo build --release --examples"
lxc exec --verbose $container -- /bin/sh -c "cd /build; /root/.cargo/bin/cargo deb --no-build"

echo "Pulling debs"
lxc file pull -r $container/build/target/debian /tmp/$container

cd /tmp/$container; find debian -regex ".*mjolnir_.*.deb" -exec rename "s/mjolnir_(.*).deb/$(echo $distro)_mjolnir_\$1.deb/g" {}  \;

cp /tmp/$container/debian/*.deb $path

finish