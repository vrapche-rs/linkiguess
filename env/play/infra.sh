#!/bin/bash

if [ "$1" == "up" ]; then
    podman kube play --replace env/kube/infra.yml
elif [ "$1" == "down" ]; then
    podman kube down env/kube/infra.yml
fi
