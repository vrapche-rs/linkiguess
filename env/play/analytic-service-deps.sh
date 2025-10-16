#!/bin/bash

if [ "$1" == "up" ]; then
    podman kube play --replace env/kube/analytic-service-deps.yml
elif [ "$1" == "down" ]; then
    podman kube down env/kube/analytic-service-deps.yml
fi
