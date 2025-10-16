#!/bin/bash

if [ "$1" == "up" ]; then
    podman build -t linkiguess-backend-service source/backend-service
    mkdir -p /tmp/linkiguess/backend-service-cargo
    podman kube play --replace env/kube/backend-service.yml
elif [ "$1" == "down" ]; then
    podman kube down env/kube/backend-service.yml
fi
