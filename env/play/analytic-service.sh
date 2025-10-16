#!/bin/bash

if [ "$1" == "up" ]; then
    podman build -t linkiguess-analytic-service source/analytic-service
    mkdir -p /tmp/linkiguess/analytic-service-cargo
    podman kube play --replace env/kube/analytic-service.yml
elif [ "$1" == "down" ]; then
    podman kube down env/kube/analytic-service.yml
fi
