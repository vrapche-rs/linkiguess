#!/bin/bash

if [ "$1" == "up" ]; then
    podman build -t linkiguess-short-service source/short-service
    mkdir -p /tmp/linkiguess/short-service-cargo
    podman kube play --replace env/kube/short-service.yml
elif [ "$1" == "down" ]; then
    podman kube down env/kube/short-service.yml
fi
