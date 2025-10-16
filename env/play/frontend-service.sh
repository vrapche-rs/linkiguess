#!/bin/bash

if [ "$1" == "up" ]; then
    podman build -t linkiguess-frontend-service source/frontend-service
    podman kube play --replace env/kube/frontend-service.yml
elif [ "$1" == "down" ]; then
    podman kube down env/kube/frontend-service.yml
fi
