#!/bin/sh

podman kube down env/kube/secrets.yml
podman kube down env/kube/analytic-service.yml
podman kube down env/kube/analytic-service-deps.yml
podman kube down env/kube/short-service-deps.yml
podman kube down env/kube/short-service.yml
podman kube down env/kube/purger-service.yml
podman kube down env/kube/frontend-service.yml
podman kube down env/kube/backend-service.yml
podman kube down env/kube/infra.yml
podman kube down env/kube/lb.yml
