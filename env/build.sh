#!/bin/sh

podman build -t linkiguess-short-service source/short-service
podman build -t linkiguess-analytic-service source/analytic-service
podman build -t linkiguess-purger-service source/purger-service
podman build -t linkiguess-frontend-service source/frontend-service
podman build -t linkiguess-backend-service source/backend-service
