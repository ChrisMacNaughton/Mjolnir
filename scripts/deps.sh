lxc="/snap/bin/lxc"
function deps() {
    distro=$1
    container=$2
    # echo "Installing deps"
    packages="cmake libssl-dev protobuf-compiler libprotobuf-dev libsodium-dev liblzma-dev pkg-config"
    # if [ "$distro" = "trusty" ]
    #     then
    #     $lxc exec --verbose $container -- /bin/sh -c "add-apt-repository -y ppa:chris-lea/libsodium"
    #     $lxc exec --verbose $container -- /bin/sh -c "add-apt-repository -y ppa:alexhuang/libzmq"
    #     packages="$packages libzmq"
    # fi
    # if [ "$distro" = "xenial" ]
    #     then
    #     packages="$packages libzmq3-dev"
    # fi
    $lxc exec --verbose $container -- /bin/sh -c "DEBIAN_FRONTEND=noninteractive apt-get update -qq"
    $lxc exec --verbose $container -- /bin/sh -c "DEBIAN_FRONTEND=noninteractive apt-get install -yqq $packages"

    echo "About to install rust"
    $lxc exec --verbose $container -- /bin/sh -c "curl https://sh.rustup.rs -sSf | sh -s -- -y"
}