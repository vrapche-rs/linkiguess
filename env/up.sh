#!/bin/sh

./env/tls-init.sh

mkdir -p /tmp/linkiguess/anaytic-service-cargo
mkdir -p /tmp/linkiguess/short-service-cargo
mkdir -p /tmp/linkiguess/purger-service-cargo
mkdir -p /tmp/linkiguess/backend-service-cargo

podman kube play --replace env/kube/secrets.yml
podman kube play --replace env/kube/analytic-service.yml
podman kube play --replace env/kube/analytic-service-deps.yml
podman kube play --replace env/kube/short-service-deps.yml
podman kube play --replace env/kube/short-service.yml
podman kube play --replace env/kube/purger-service.yml
podman kube play --replace env/kube/frontend-service.yml
podman kube play --replace env/kube/backend-service.yml
podman kube play --replace env/kube/infra.yml
podman kube play --replace env/kube/lb.yml
