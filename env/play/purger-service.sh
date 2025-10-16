#!/bin/bash

if [ "$1" == "up" ]; then
    podman build -t linkiguess-purger-service source/purger-service
    mkdir -p /tmp/linkiguess/purger-service-cargo
    podman kube play --replace env/kube/purger-service.yml
elif [ "$1" == "down" ]; then
    podman kube down env/kube/purger-service.yml
fi
