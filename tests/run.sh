#!/bin/bash

set -euo pipefail
SCRIPTPATH="$( cd "$(dirname "$0")" ; pwd -P )"

. $SCRIPTPATH/../scripts/deps.sh
. $SCRIPTPATH/../scripts/helpers.sh

function finish {
    # echo "Cleaning up: ($?)!"
    lxc delete -f master > /dev/null 2>&1  || true
    for i in $(seq 1 3);
    do
        lxc delete -f agent-$i > /dev/null 2>&1 || true
    done
    unset master_ip
    # echo "finished cleaning up"
}
trap finish EXIT

echo "Setting up testing environment"
echo "=============================="
echo

if [ -n ${VERBOSE+x} ];
then
    echo "Spawning Master"
fi
lxc launch ubuntu:xenial master
sleep 5
master_ip=`lxc list "master" -c 4 | awk '!/IPV4/{ if ( $2 != "" ) print $2}'`
export master_ip
if [ -n ${VERBOSE+x} ];
then
    echo "Installing dependencies on Master"
fi

deps xenial master > /dev/null

if [ -n ${VERBOSE+x} ];
then
    echo "Setting up Master for build"
fi

lxc exec master -- /bin/sh -c "/bin/mkdir -p /build"
# echo "Pushing files into container"
tar --exclude-vcs --exclude=target -zcf - . | lxc exec --verbose master -- /bin/sh -c "/bin/tar zxf - -C /build"

lxc_exec master "cd /build/mjolnird; /root/.cargo/bin/cargo build --all"  > /dev/null
lxc_exec master "cd /build; /root/.cargo/bin/cargo build --examples"  > /dev/null

cat > config.toml <<EOF
[mjolnir]
  key_path = "/usr/local/share/mjolnir"
  masters = ["$(echo $master_ip):11011:12011"]
  plugin_path = "/build/target/debug/examples"
  secret = "w[4957ha[ruognqp357gf;eruigap47gfa;IRYEgf0a864fo"

[master]
bind = "0.0.0.0:11011:12011"

[agent]
bind = "0.0.0.0:11012:12012"

[[pipelines]]

  [[pipelines.actions]]
    plugin = "clean_disk"

  [pipelines.trigger]
    type = "alertmanager"
    name = "full-disk"
EOF

lxc file push -p ./config.toml master/usr/local/share/mjolnir/config.toml
lxc file push -p ./systemd/mjolnird-master.service master/etc/systemd/system/mjolnird-master.service

rm config.toml

# echo "Starting mjolnird-master service"
lxc_exec master "ln -s /build/target/debug/mjolnird /usr/sbin/mjolnird || true"
lxc_exec master "systemctl start mjolnird-master"

if [ -n ${VERBOSE+x} ];
then
    echo "Spawning agents"
fi

for i in $(seq 1 3)
do
    # echo "Setting up agent-$i"
    lxc copy master agent-$i || true
    lxc start agent-$i || true
    lxc file push ./systemd/mjolnird-agent.service agent-$i/etc/systemd/system/mjolnird-agent.service
    lxc_exec agent-$i "systemctl start mjolnird-agent"
done

echo "====================================="
echo "Running tests"
echo "====================================="
echo

failed=0
for test_file in $SCRIPTPATH/test_*;
do
    status=0
    res=$(. $test_file) || status=$? && true
    if [ -z ${VERBOSE+x} ];
    then
        printf .
    else
        printf $test_file | sed 's/\.\/tests\/test_//'
        if [ "$status" = 0 ];
        then
            echo " - Success"
        else
            echo " - Failure"
            echo "    $res"
            failed=1
        fi
    fi
done

unset master_ip

if [ -n ${VERBOSE+x} ];
then
    echo
fi

if [ "$failed" = 0 ];
then
    echo "Success!"
else
    echo "Failure!"
    exit 1
fi