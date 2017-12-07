function lxc_exec() {
    if [ -z "$1" ]
    then
        echo "No argument supplied, requires container"
        exit 1
    fi
    if [ -z "$2" ]
    then
        echo "No 2nd argument supplied, command"
        exit 1
    fi

    container=$1
    command=$2

    lxc exec --verbose $container -- /bin/sh -c "$command"
}