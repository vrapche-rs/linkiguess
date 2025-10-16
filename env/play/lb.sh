#!/bin/bash

if [ "$1" == "up" ]; then
    ./env//tls-init.sh
    podman kube play --replace env/kube/lb.yml
elif [ "$1" == "down" ]; then
    podman kube down env/kube/lb.yml
fi
